//! Kubernetes driver for dynamic challenge instances: Pods, Services,
//! NetworkPolicies, attach/exec.
//!
//! On [`init`], the client connects (in-cluster or kubeconfig), ensures the
//! target namespace exists, and reconciles egress policies so pods labeled
//! `cds/internet=false` cannot reach the public internet except through
//! controlled rules, while `cds/internet=true` pods receive DNS + user-defined
//! exceptions.

pub mod lifecycle;
pub mod metadata;
pub mod status;
/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Defines the `util` submodule (see sibling `*.rs` files).
mod util;

/// Defines the `worker` submodule (see sibling `*.rs` files).
pub mod worker;

use std::{collections::BTreeMap, path::Path, process};

use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use cds_checker::Checker;
use cds_db::challenge::Port;
use cds_env::Env;
use futures_util::{SinkExt, StreamExt as _, stream::SplitStream};
pub use k8s_openapi;
use k8s_openapi::{
    api::{
        core::v1::{
            Container as K8sContainer, ContainerPort, EnvVar, Namespace, Pod, PodSpec,
            ResourceRequirements, Service, ServicePort, ServiceSpec,
        },
        networking::v1::{
            IPBlock, NetworkPolicy, NetworkPolicyEgressRule, NetworkPolicyPeer, NetworkPolicyPort,
            NetworkPolicySpec,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity,
        apis::meta::v1::{LabelSelector, ObjectMeta, OwnerReference},
        util::intstr::IntOrString,
    },
};
pub use kube;
use kube::{
    Client as K8sClient, Config as K8sConfig,
    api::{Api, AttachParams, DeleteParams, ListParams, Patch, PatchParams, PostParams},
    config::{KubeConfigOptions, Kubeconfig},
};
use metadata::{InstanceIdentity, InstanceMetadata, InstanceMode};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio_util::{
    codec::{BytesCodec, Framed, FramedRead},
    sync::CancellationToken,
};
use tracing::{error, info};

use crate::traits::ClusterError;

/// Connected API client, target namespace, ingress mode (`Expose` vs `Proxy`),
/// and checker for env generation.
#[derive(Clone)]
pub struct Cluster {
    client: K8sClient,
    namespace: String,
    traffic: cds_env::cluster::Traffic,

    checker: Checker,
}

/// Builds the Kubernetes client, ensures namespace + baseline NetworkPolicies,
/// and spawns garbage-collection worker.
pub async fn init(env: &Env, checker: &Checker) -> Result<Cluster, ClusterError> {
    let client = if env.cluster.auto_infer {
        K8sClient::try_from(K8sConfig::infer().await?)?
    } else {
        let kube_config = Kubeconfig::read_from(Path::new(&env.cluster.config_path))?;
        K8sClient::try_from(
            K8sConfig::from_custom_kubeconfig(kube_config, &KubeConfigOptions::default()).await?,
        )?
    };
    if client.apiserver_version().await.is_err() {
        error!("Failed to connect to Kubernetes API server.");
        process::exit(1);
    }
    info!("Kubernetes client initialized successfully.");

    let namespace_api: Api<Namespace> = Api::all(client.clone());
    let namespaces = namespace_api.list(&ListParams::default()).await?;
    if !namespaces
        .items
        .iter()
        .any(|namespace| namespace.metadata.name == Some(env.cluster.namespace.to_owned()))
    {
        let namespace = Namespace {
            metadata: ObjectMeta {
                name: Some(env.cluster.namespace.to_owned()),
                ..Default::default()
            },
            ..Default::default()
        };
        let _ = namespace_api
            .create(&PostParams::default(), &namespace)
            .await;
        info!("Namespace is created successfully.");
    }

    let network_policy_api: Api<NetworkPolicy> =
        Api::namespaced(client.clone(), env.cluster.namespace.as_str());

    if network_policy_api
        .get("cds-internet-restricted")
        .await
        .is_err()
    {
        let network_policy = NetworkPolicy {
            metadata: ObjectMeta {
                name: Some("cds-internet-restricted".to_owned()),
                namespace: Some(env.cluster.namespace.to_owned()),
                ..Default::default()
            },
            spec: Some(NetworkPolicySpec {
                pod_selector: Some(LabelSelector {
                    match_labels: Some(BTreeMap::from([(
                        "cds/internet".to_owned(),
                        "false".to_owned(),
                    )])),
                    ..Default::default()
                }),
                policy_types: Some(vec!["Egress".to_owned()]),
                ..Default::default()
            }),
        };
        network_policy_api
            .create(&PostParams::default(), &network_policy)
            .await?;

        info!("Restricted network policy is created successfully.");
    }

    let desired_excluded_cidrs = env.cluster.egress_excluded_cidrs.clone();
    let network_policy = NetworkPolicy {
        metadata: ObjectMeta {
            name: Some("cds-internet-allowed".to_owned()),
            namespace: Some(env.cluster.namespace.to_owned()),
            ..Default::default()
        },
        spec: Some(NetworkPolicySpec {
            pod_selector: Some(LabelSelector {
                match_labels: Some(BTreeMap::from([(
                    "cds/internet".to_owned(),
                    "true".to_owned(),
                )])),
                ..Default::default()
            }),
            policy_types: Some(vec!["Egress".to_owned()]),
            egress: Some(vec![
                NetworkPolicyEgressRule {
                    to: Some(vec![NetworkPolicyPeer {
                        namespace_selector: Some(LabelSelector {
                            match_labels: Some(BTreeMap::from([(
                                "kubernetes.io/metadata.name".to_string(),
                                "kube-system".to_string(),
                            )])),
                            ..Default::default()
                        }),
                        pod_selector: Some(LabelSelector {
                            match_labels: Some(BTreeMap::from([(
                                "k8s-app".to_string(),
                                "kube-dns".to_string(),
                            )])),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }]),
                    ports: Some(vec![
                        NetworkPolicyPort {
                            protocol: Some("UDP".to_string()),
                            port: Some(IntOrString::Int(53)),
                            ..Default::default()
                        },
                        NetworkPolicyPort {
                            protocol: Some("TCP".to_string()),
                            port: Some(IntOrString::Int(53)),
                            ..Default::default()
                        },
                    ]),
                    ..Default::default()
                },
                NetworkPolicyEgressRule {
                    to: Some(vec![NetworkPolicyPeer {
                        ip_block: Some(IPBlock {
                            cidr: "0.0.0.0/0".to_owned(),
                            except: Some(desired_excluded_cidrs.clone()),
                        }),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }),
    };
    match network_policy_api.get("cds-internet-allowed").await {
        Err(_) => {
            network_policy_api
                .create(&PostParams::default(), &network_policy)
                .await?;
            info!("Allowed network policy is created successfully.");
        }
        Ok(np) => {
            let current_excluded = np
                .spec
                .as_ref()
                .and_then(|s| s.egress.as_ref())
                .and_then(|egress| egress.first())
                .and_then(|rule| rule.to.as_ref())
                .and_then(|to| to.first())
                .and_then(|peer| peer.ip_block.as_ref())
                .and_then(|ipb| ipb.except.clone());

            if current_excluded != Some(desired_excluded_cidrs.clone()) {
                network_policy_api
                    .patch(
                        "cds-internet-allowed",
                        &PatchParams::default(),
                        &Patch::Merge(&network_policy),
                    )
                    .await?;

                info!("Allowed network policy updated due to excluded CIDRs change.");
            }
        }
    }

    let cluster = Cluster {
        client,
        namespace: env.cluster.namespace.clone(),
        traffic: env.cluster.traffic.clone(),

        checker: checker.clone(),
    };

    worker::cleaner(cluster.clone()).await;

    Ok(cluster)
}

impl Cluster {
    /// Returns service.
    pub async fn get_service(&self, id: &str) -> Result<Service, ClusterError> {
        metadata::validate_id(id)?;
        let service = self
            .get_services_by_label(&format!("cds/instance_id={}", id))
            .await?
            .first()
            .ok_or(ClusterError::NotFound("service_not_found".to_owned()))?
            .to_owned();

        Ok(service)
    }

    /// Returns services by label.

    pub async fn get_services_by_label(&self, label: &str) -> Result<Vec<Service>, ClusterError> {
        let service_api: Api<Service> =
            Api::namespaced(self.client.clone(), self.namespace.as_str());

        let services = service_api
            .list(&ListParams {
                label_selector: Some(metadata::selector(label)),
                ..Default::default()
            })
            .await?;

        Ok(services.items)
    }

    /// Creates service.

    pub async fn create_service(&self, service: Service) -> Result<Service, ClusterError> {
        let service_api: Api<Service> =
            Api::namespaced(self.client.clone(), self.namespace.as_str());

        let service = service_api.create(&Default::default(), &service).await?;

        Ok(service)
    }

    /// Deletes service.

    pub async fn delete_service(&self, id: &str) -> Result<(), ClusterError> {
        metadata::validate_id(id)?;
        let service_api: Api<Service> =
            Api::namespaced(self.client.clone(), self.namespace.as_str());

        let _ = service_api
            .delete_collection(
                &DeleteParams::default(),
                &ListParams::default()
                    .labels(&metadata::selector(&format!("cds/instance_id={id}"))),
            )
            .await?;

        Ok(())
    }

    /// Returns pod.

    pub async fn get_pod(&self, id: &str) -> Result<Pod, ClusterError> {
        metadata::validate_id(id)?;
        let pod = self
            .get_pods_by_label(&format!("cds/instance_id={}", id))
            .await?
            .first()
            .ok_or(ClusterError::NotFound("pod_not_found".to_owned()))?
            .to_owned();

        Ok(pod)
    }

    /// Returns pods list.

    pub async fn get_pods_list(&self) -> Result<Vec<Pod>, ClusterError> {
        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), self.namespace.as_str());

        let pods = pod_api
            .list(&ListParams::default().labels(metadata::MANAGED_SELECTOR))
            .await?;

        Ok(pods.items)
    }

    /// Returns pods by label.

    pub async fn get_pods_by_label(&self, label: &str) -> Result<Vec<Pod>, ClusterError> {
        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), self.namespace.as_str());

        let pods = pod_api
            .list(&ListParams {
                label_selector: Some(metadata::selector(label)),
                ..Default::default()
            })
            .await?;

        Ok(pods.items)
    }

    /// Lists pods that still reserve an instance slot, including recovery and
    /// deletion.
    pub async fn get_active_pods_by_label(&self, label: &str) -> Result<Vec<Pod>, ClusterError> {
        Ok(self
            .get_pods_by_label(label)
            .await?
            .into_iter()
            .filter(|pod| status::observe(pod).state.counts_toward_quota())
            .collect())
    }

    /// Lists all managed services once for response projection and orphan
    /// cleanup.
    pub async fn get_services_list(&self) -> Result<Vec<Service>, ClusterError> {
        self.get_services_by_label("").await
    }

    /// Creates pod.

    pub async fn create_pod(&self, pod: Pod) -> Result<Pod, ClusterError> {
        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), self.namespace.as_str());

        let pod = pod_api.create(&Default::default(), &pod).await?;

        Ok(pod)
    }

    /// Stops the observed pod and leaves a grace period for orderly
    /// termination.
    pub async fn delete_pod(&self, id: &str) -> Result<(), ClusterError> {
        match self.get_pod(id).await {
            Ok(pod) => {
                lifecycle::stop(&Api::namespaced(self.client.clone(), &self.namespace), &pod).await
            }
            Err(ClusterError::NotFound(_)) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Rejects a stale cleaner snapshot if an instance was renewed meanwhile.
    pub async fn delete_expired_instance(&self, pod: &Pod, now: i64) -> Result<bool, ClusterError> {
        let metadata = InstanceMetadata::parse(pod)?;
        if lifecycle::delete_if_expired(
            &Api::namespaced(self.client.clone(), &self.namespace),
            pod,
            now,
        )
        .await?
        {
            self.delete_service(&metadata.identity.id).await?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Creates challenge instance.

    pub async fn create_challenge_instance(
        &self,
        user: cds_db::UserAccountView,
        team: Option<cds_db::TeamView>,
        game: Option<cds_db::GameDetail>,
        challenge: cds_db::ChallengeDetail,
        mode: InstanceMode,
    ) -> Result<String, ClusterError> {
        let id = util::gen_safe_nanoid();
        let user_id = user.id;
        let challenge_id = challenge.id;
        let game_id = game.as_ref().map(|game| game.id);
        let team_id = team.as_ref().map(|team| team.id);
        let name = format!("cds-{}", id);

        let instance = challenge
            .clone()
            .instance
            .ok_or_else(|| ClusterError::MissingEnvConfiguration)?;

        let all_ports = instance
            .containers
            .iter()
            .flat_map(|container| container.ports.clone())
            .collect::<Vec<Port>>();

        let identity = InstanceIdentity {
            id: id.clone(),
            creator_user_id: user.id,
            challenge_id: challenge.id,
            team_id,
            game_id,
            mode,
        };
        let metadata = ObjectMeta {
            name: Some(name.clone()),
            labels: Some(identity.labels(instance.internet)?),
            annotations: Some(InstanceMetadata::annotations(instance.duration)?),
            ..Default::default()
        };

        let subject = if let (Some(_), Some(team)) = (game, team) {
            cds_checker::Subject::Team(team.id)
        } else {
            cds_checker::Subject::User(user.id)
        };

        let checker_environ = self.checker.generate(&challenge, subject).await?;

        let checker_env_vars = checker_environ
            .into_iter()
            .map(|(k, v)| EnvVar {
                name: k,
                value: Some(v),
                ..Default::default()
            })
            .collect::<Vec<EnvVar>>();

        let pod = Pod {
            metadata: metadata.clone(),
            spec: Some(PodSpec {
                containers: instance
                    .containers
                    .into_iter()
                    .map(|container| {
                        let merged_env_vars = container
                            .envs
                            .into_iter()
                            .map(|env_var| EnvVar {
                                name: env_var.key,
                                value: Some(env_var.value),
                                ..Default::default()
                            })
                            .chain(checker_env_vars.clone())
                            .collect::<Vec<EnvVar>>();

                        K8sContainer {
                            name: format!("cds-{}", util::gen_safe_nanoid()),
                            image: Some(container.image),
                            env: Some(merged_env_vars),
                            ports: Some(
                                container
                                    .ports
                                    .into_iter()
                                    .map(|port| ContainerPort {
                                        container_port: port.port,
                                        protocol: Some(port.protocol),
                                        ..Default::default()
                                    })
                                    .collect::<Vec<ContainerPort>>(),
                            ),
                            image_pull_policy: Some(container.image_pull_policy),
                            resources: Some(ResourceRequirements {
                                requests: Some(
                                    [
                                        ("cpu", "10m".to_owned()),
                                        ("memory", "32Mi".to_owned()),
                                        ("ephemeral-storage", "64Mi".to_owned()),
                                    ]
                                    .iter()
                                    .cloned()
                                    .map(|(k, v)| (k.to_owned(), Quantity(v)))
                                    .collect(),
                                ),
                                limits: Some(
                                    [
                                        ("cpu", container.cpu_limit.to_string()),
                                        ("memory", format!("{}Mi", container.memory_limit)),
                                    ]
                                    .iter()
                                    .cloned()
                                    .map(|(k, v)| (k.to_owned(), Quantity(v)))
                                    .collect(),
                                ),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
                    })
                    .collect::<Vec<K8sContainer>>(),
                ..Default::default()
            }),
            ..Default::default()
        };

        let pod = self.create_pod(pod).await?;

        let service_type = match self.traffic {
            cds_env::cluster::Traffic::Expose => "NodePort",
            cds_env::cluster::Traffic::Proxy => "ClusterIP",
        };

        let service = Service {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                labels: metadata.labels.clone(),
                owner_references: Some(vec![OwnerReference {
                    api_version: "v1".to_owned(),
                    kind: "Pod".to_owned(),
                    name: name.clone(),
                    uid: pod
                        .metadata
                        .uid
                        .clone()
                        .ok_or_else(|| metadata::invalid("uid"))?,
                    controller: Some(true),
                    block_owner_deletion: Some(false),
                }]),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                selector: Some(BTreeMap::from([(
                    "cds/instance_id".to_owned(),
                    id.to_string(),
                )])),
                ports: Some(
                    all_ports
                        .into_iter()
                        .map(|port| ServicePort {
                            name: Some(port.port.to_string()),
                            port: port.port,
                            target_port: None,
                            protocol: Some(port.protocol),
                            ..Default::default()
                        })
                        .collect::<Vec<ServicePort>>(),
                ),
                type_: Some(service_type.to_owned()),
                ..Default::default()
            }),
            ..Default::default()
        };

        if let Err(error) = self.create_service(service).await {
            // Attempt both cleanups, but retain the original provisioning
            // error.
            if let Err(cleanup) = self.delete_challenge_instance(&id).await {
                tracing::warn!(instance_id = %id, error = ?cleanup, "failed to roll back instance");
            }
            return Err(error);
        }

        tracing::info!(
            resource = "instance",
            action = "create",
            instance_id = %id,
            user_id,
            challenge_id,
            game_id,
            team_id,
            "resource step completed",
        );
        Ok(id)
    }

    /// Renews an authorized snapshot using conflict detection and fresh rule
    /// checks.
    pub async fn renew_challenge_instance(&self, pod: &Pod) -> Result<Pod, ClusterError> {
        let renewed =
            lifecycle::renew(&Api::namespaced(self.client.clone(), &self.namespace), pod).await?;
        tracing::info!(instance_id = ?renewed.metadata.name, resource = "instance", action = "renew", "resource step completed");
        Ok(renewed)
    }

    /// Stop exactly the authorized pod UID before removing its network entry.
    pub async fn stop_challenge_instance(&self, pod: &Pod) -> Result<(), ClusterError> {
        let identity = InstanceIdentity::parse(&pod.metadata)?;
        lifecycle::stop(&Api::namespaced(self.client.clone(), &self.namespace), pod).await?;
        self.delete_service(&identity.id).await?;
        Ok(())
    }

    /// Deletes challenge instance.

    pub async fn delete_challenge_instance(&self, id: &str) -> Result<(), ClusterError> {
        metadata::validate_id(id)?;
        let pod_result = self.delete_pod(id).await;
        let service_result = self.delete_service(id).await;
        pod_result?;
        service_result?;
        tracing::info!(
            instance_id = id,
            resource = "instance",
            action = "stop",
            "instance stop requested"
        );
        Ok(())
    }

    /// Proxies WebSocket traffic into a pod port via `wsrx`.
    pub async fn wsrx(&self, id: &str, port: u16, ws: WebSocket) -> Result<(), ClusterError> {
        let name = format!("cds-{}", id);

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), self.namespace.as_str());
        let mut pf = pod_api.portforward(&name, &[port]).await?;
        let pfw = pf.take_stream(port);
        if let Some(pfw) = pfw {
            let stream = Framed::new(pfw, wsrx::proxy::MessageCodec::new());
            let ws: wsrx::WrappedWsStream = ws.into();
            let cancel_token = CancellationToken::new();
            wsrx::proxy::proxy_stream(stream, ws, cancel_token).await?;
        }
        Ok(())
    }

    /// Attaches to a container and streams a shell session over WebSocket.
    pub async fn exec(
        &self,
        id: &str,
        container_id: &str,
        command: String,
        ws: WebSocket,
    ) -> Result<(), ClusterError> {
        /// Copies WebSocket text frames into the pod stdin stream.
        async fn process_client_to_pod<W>(
            mut receiver: SplitStream<WebSocket>,
            mut stdin_writer: W,
        ) where
            W: AsyncWrite + Unpin + Sized, {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(text) => {
                        if stdin_writer.write_all(text.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
            let _ = stdin_writer.shutdown().await;
        }

        /// Streams pod stdout back to the WebSocket client.
        async fn process_pod_to_client<R, S>(stdout_reader: R, mut sender: S)
        where
            R: AsyncRead + Unpin,
            S: SinkExt<Message> + Unpin, {
            let mut reader = FramedRead::new(stdout_reader, BytesCodec::new());
            while let Some(result) = reader.next().await {
                match result {
                    Ok(bytes) => {
                        if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                            if sender
                                .send(Message::Text(Utf8Bytes::from(text)))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            let _ = sender.close().await;
        }

        let (sender, receiver) = ws.split();
        let name = format!("cds-{}", id);

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), self.namespace.as_str());

        let attach_params = AttachParams {
            container: Some(format!("cds-{}", container_id)),
            stdin: true,
            stdout: true,
            stderr: false,
            tty: true,
            ..Default::default()
        };

        let mut attached = pod_api.exec(&name, vec![command], &attach_params).await?;

        // SAFETY: `stdin` and `stdout` are guaranteed to be unwrapped.
        let stdin_writer = attached.stdin().unwrap();
        let stdout_reader = BufReader::new(attached.stdout().unwrap());

        let mut recv_task = tokio::spawn(async move {
            process_client_to_pod(receiver, stdin_writer).await;
        });

        let mut send_task = tokio::spawn(async move {
            process_pod_to_client(stdout_reader, sender).await;
        });

        tokio::select! {
            _ = &mut recv_task => {
                send_task.abort();
            },
            _ = &mut send_task => {
                recv_task.abort();
            },
        }

        Ok(())
    }
}

pub mod traits;
mod util;
pub mod worker;

use std::{collections::BTreeMap, path::Path, process};

use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use cds_db::entity::challenge::Port;
use futures_util::{SinkExt, StreamExt, stream::SplitStream};
pub use k8s_openapi;
use k8s_openapi::{
    api::{
        core::v1::{
            Container as K8sContainer, ContainerPort, EnvVar, Namespace, Pod, PodSpec,
            ResourceRequirements, Service, ServicePort, ServiceSpec,
        },
        networking::v1::{NetworkPolicy, NetworkPolicySpec},
    },
    apimachinery::pkg::{
        api::resource::Quantity,
        apis::meta::v1::{LabelSelector, ObjectMeta},
    },
};
pub use kube;
use kube::{
    Client as K8sClient, Config as K8sConfig, ResourceExt,
    api::{Api, AttachParams, DeleteParams, ListParams, Patch, PatchParams, PostParams},
    config::{KubeConfigOptions, Kubeconfig},
};
use once_cell::sync::OnceCell;
use serde_json::json;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio_util::{
    codec::{BytesCodec, Framed, FramedRead},
    sync::CancellationToken,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::traits::{ClusterError, Nat};

static K8S_CLIENT: OnceCell<K8sClient> = OnceCell::new();

pub fn get_k8s_client() -> K8sClient {
    K8S_CLIENT.get().unwrap().clone()
}

pub async fn init() -> Result<(), ClusterError> {
    let client = if cds_env::get_config().cluster.auto_infer {
        K8sClient::try_from(K8sConfig::infer().await?)?
    } else {
        let kube_config =
            Kubeconfig::read_from(Path::new(&cds_env::get_config().cluster.config_path))?;
        K8sClient::try_from(
            K8sConfig::from_custom_kubeconfig(kube_config, &KubeConfigOptions::default()).await?,
        )?
    };
    if client.apiserver_version().await.is_err() {
        error!("Failed to connect to Kubernetes API server.");
        process::exit(1);
    }
    let _ = K8S_CLIENT.set(client);
    info!("Kubernetes client initialized successfully.");

    let namespace_api: Api<Namespace> = Api::all(get_k8s_client());
    let namespaces = namespace_api.list(&ListParams::default()).await?;
    if !namespaces.items.iter().any(|namespace| {
        namespace.metadata.name == Some(cds_env::get_config().cluster.namespace.to_owned())
    }) {
        let namespace = Namespace {
            metadata: ObjectMeta {
                name: Some(cds_env::get_config().cluster.namespace.to_owned()),
                ..Default::default()
            },
            ..Default::default()
        };
        let _ = namespace_api
            .create(&PostParams::default(), &namespace)
            .await;
        info!("Namespace is created successfully.");
    }

    let network_policy_api: Api<NetworkPolicy> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );
    if network_policy_api
        .get("cds-internet-restricted")
        .await
        .is_err()
    {
        let network_policy = NetworkPolicy {
            metadata: ObjectMeta {
                name: Some("cds-internet-restricted".to_owned()),
                namespace: Some(cds_env::get_config().cluster.namespace.to_owned()),
                ..Default::default()
            },
            spec: Some(NetworkPolicySpec {
                pod_selector: LabelSelector {
                    match_labels: Some(BTreeMap::from([(
                        "cds/internet".to_owned(),
                        "false".to_owned(),
                    )])),
                    ..Default::default()
                },
                policy_types: Some(vec!["Egress".to_owned()]),
                ..Default::default()
            }),
        };
        network_policy_api
            .create(&PostParams::default(), &network_policy)
            .await?;
        info!("Network policy is created successfully.");
    }

    worker::cleaner().await;

    Ok(())
}

pub async fn get_service(id: Uuid) -> Result<Service, ClusterError> {
    let service = get_services_by_label(&format!("cds/env_id={}", id))
        .await?
        .first()
        .ok_or(ClusterError::NotFound("service_not_found".to_owned()))?
        .to_owned();

    Ok(service)
}

pub async fn get_services_by_label(label: &str) -> Result<Vec<Service>, ClusterError> {
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let services = service_api
        .list(&ListParams {
            label_selector: Some(label.to_owned()),
            ..Default::default()
        })
        .await?;

    Ok(services.items)
}

pub async fn create_service(service: Service) -> Result<Service, ClusterError> {
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let service = service_api.create(&Default::default(), &service).await?;

    Ok(service)
}

pub async fn delete_service(id: &str) -> Result<(), ClusterError> {
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let _ = service_api
        .delete_collection(
            &DeleteParams::default(),
            &ListParams::default().labels(&format!("cds/env_id={id}")),
        )
        .await?;

    Ok(())
}

pub async fn get_pod(id: &str) -> Result<Pod, ClusterError> {
    let pod = get_pods_by_label(&format!("cds/env_id={}", id))
        .await?
        .first()
        .ok_or(ClusterError::NotFound("pod_not_found".to_owned()))?
        .to_owned();

    Ok(pod)
}

pub async fn get_pods_list() -> Result<Vec<Pod>, ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let pods = pod_api.list(&ListParams::default()).await?;

    Ok(pods.items)
}

pub async fn get_pods_by_label(label: &str) -> Result<Vec<Pod>, ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let pods = pod_api
        .list(&ListParams {
            label_selector: Some(label.to_owned()),
            field_selector: Some(
                "status.phase!=Succeeded,status.phase!=Failed,status.phase!=Unknown".to_owned(),
            ),
            ..Default::default()
        })
        .await?;

    Ok(pods.items)
}

pub async fn create_pod(pod: Pod) -> Result<Pod, ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let pod = pod_api.create(&Default::default(), &pod).await?;

    Ok(pod)
}

pub async fn delete_pod(id: &str) -> Result<(), ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let _ = pod_api
        .delete_collection(
            &DeleteParams {
                grace_period_seconds: Some(0),
                ..Default::default()
            },
            &ListParams::default().labels(&format!("cds/env_id={id}")),
        )
        .await?;

    Ok(())
}

pub async fn create_challenge_env(
    user: cds_db::entity::user::Model,
    team: Option<cds_db::entity::team::Model>,
    game: Option<cds_db::entity::game::Model>,
    challenge: cds_db::entity::challenge::Model,
) -> Result<(), ClusterError> {
    let id = util::gen_safe_nanoid();
    let name = format!("cds-{}", id);

    let env = challenge.clone().env.unwrap();

    let all_ports = env
        .containers
        .iter()
        .flat_map(|container| container.ports.clone())
        .collect::<Vec<Port>>();

    let metadata = ObjectMeta {
        name: Some(name.clone()),
        labels: Some(BTreeMap::from([
            ("cds/app".to_owned(), "challenges".to_owned()),
            ("cds/env_id".to_owned(), id.to_string()),
            ("cds/internet".to_owned(), format!("{}", env.internet)),
            ("cds/user_id".to_owned(), format!("{}", user.id)),
            (
                "cds/team_id".to_owned(),
                format!(
                    "{}",
                    match &team {
                        Some(team) => team.id,
                        _ => 0,
                    }
                ),
            ),
            (
                "cds/game_id".to_owned(),
                format!(
                    "{}",
                    match &game {
                        Some(game) => game.id,
                        _ => 0,
                    }
                ),
            ),
            ("cds/challenge_id".to_owned(), format!("{}", challenge.id)),
        ])),
        annotations: Some(BTreeMap::from([
            ("cds/challenge".to_owned(), json!(challenge).to_string()),
            ("cds/user".to_owned(), json!(user).to_string()),
            ("cds/team".to_owned(), json!(team).to_string()),
            ("cds/game".to_owned(), json!(game).to_string()),
            ("cds/renew".to_owned(), format!("{}", 0)),
            ("cds/duration".to_owned(), format!("{}", env.duration)),
            ("cds/ports".to_owned(), json!(all_ports).to_string()),
        ])),
        ..Default::default()
    };

    let operator_id = if let (Some(_), Some(team)) = (game, team) {
        team.id
    } else {
        user.id
    };

    let checker_environ = cds_checker::generate(&challenge, operator_id).await?;

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
            containers: env
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
                        image_pull_policy: Some("Always".to_owned()),
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

    let mut pod = create_pod(pod).await?;

    let service_type = match cds_env::get_config().cluster.traffic {
        cds_env::cluster::Traffic::Expose => "NodePort",
        cds_env::cluster::Traffic::Proxy => "ClusterIP",
    };

    let service = Service {
        metadata: metadata.clone(),
        spec: Some(ServiceSpec {
            selector: Some(BTreeMap::from([("cds/env_id".to_owned(), id.to_string())])),
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

    let service = match create_service(service).await {
        Ok(service) => service,
        Err(err) => {
            delete_challenge_env(&id.to_string()).await?;
            return Err(err);
        }
    };

    let mut nats: Vec<Nat> = vec![];

    if let Some(spec) = service.spec {
        if let Some(ports) = spec.ports {
            for port in ports {
                if let (Some(node_port), Some(protocol)) = (port.node_port, port.protocol) {
                    nats.push(Nat {
                        port: port.port,
                        protocol: protocol.to_string(),
                        node_port,
                    });
                }
            }
        }
    }

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let annotations = pod.annotations_mut();
    annotations.insert("cds/nats".to_owned(), json!(nats).to_string());

    pod_api
        .patch(
            &name,
            &PatchParams::default(),
            &Patch::Merge(json!({
                "metadata": {
                    "annotations": annotations,
                }
            })),
        )
        .await?;

    Ok(())
}

pub async fn renew_challenge_env(id: &str) -> Result<(), ClusterError> {
    let name = format!("cds-{}", id);
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let mut pod = get_pod(id).await?;

    let annotations = pod.annotations_mut();

    if let Some(renew) = annotations.get_mut("cds/renew") {
        *renew = format!("{}", renew.parse::<i64>().unwrap_or(0) + 1);
    }

    pod_api
        .patch(
            &name,
            &PatchParams::default(),
            &Patch::Merge(json!({
                "metadata": {
                    "annotations": annotations,
                }
            })),
        )
        .await?;

    Ok(())
}

pub async fn delete_challenge_env(id: &str) -> Result<(), ClusterError> {
    delete_pod(id).await?;
    delete_service(id).await?;

    Ok(())
}

pub async fn wsrx(id: &str, port: u16, ws: WebSocket) -> Result<(), ClusterError> {
    let name = format!("cds-{}", id);

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );
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

pub async fn exec(
    id: &str,
    container_id: &str,
    command: String,
    ws: WebSocket,
) -> Result<(), ClusterError> {
    async fn process_client_to_pod<W>(mut receiver: SplitStream<WebSocket>, mut stdin_writer: W)
    where
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

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_config().cluster.namespace.as_str(),
    );

    let attach_params = AttachParams {
        container: Some(format!("cds-{}", container_id)),
        stdin: true,
        stdout: true,
        stderr: false,
        tty: true,
        ..Default::default()
    };

    let mut attached = pod_api.exec(&name, vec![command], &attach_params).await?;

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

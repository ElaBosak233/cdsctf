pub mod traits;
pub mod worker;

use std::{collections::BTreeMap, fmt::format, path::Path, process};

use axum::extract::ws::WebSocket;
use cds_db::get_db;
use k8s_openapi::{
    api::core::v1::{
        Container as K8sContainer, ContainerPort, EnvVar, Namespace, Pod, PodSpec,
        ResourceRequirements, Service, ServicePort, ServiceSpec,
    },
    apimachinery::pkg::{api::resource::Quantity, apis::meta::v1::ObjectMeta},
    serde_json::json,
};
use kube::{
    Client as K8sClient, Config as K8sConfig, ResourceExt,
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams},
    config::Kubeconfig,
    runtime::{wait::conditions, watcher},
};
use once_cell::sync::OnceCell;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    EntityTrait, IntoActiveModel,
};
use tokio_util::codec::Framed;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::traits::ClusterError;

static K8S_CLIENT: OnceCell<K8sClient> = OnceCell::new();

pub fn get_k8s_client() -> K8sClient {
    K8S_CLIENT.get().unwrap().clone()
}

pub async fn init() -> Result<(), ClusterError> {
    let result = K8sConfig::from_custom_kubeconfig(
        Kubeconfig::read_from(Path::new(
            cds_config::get_config().cluster.kube_config_path.as_str(),
        ))?,
        &Default::default(),
    )
    .await;
    if let Err(e) = result {
        error!(
            "Failed to create Kubernetes client from custom config: {:?}",
            e
        );
        process::exit(1);
    }
    let config = result?;
    let client = K8sClient::try_from(config)?;
    if let Err(_) = client.apiserver_version().await {
        error!("Failed to connect to Kubernetes API server.");
        process::exit(1);
    }
    let _ = K8S_CLIENT.set(client);
    info!("Kubernetes client initialized successfully.");

    let namespace_api: Api<Namespace> = Api::all(get_k8s_client().clone());
    let namespaces = namespace_api.list(&ListParams::default()).await?;
    if !namespaces.items.iter().any(|namespace| {
        namespace.metadata.name == Some(cds_config::get_config().clone().cluster.namespace)
    }) {
        let namespace = Namespace {
            metadata: ObjectMeta {
                name: Some(cds_config::get_config().clone().cluster.namespace),
                ..Default::default()
            },
            ..Default::default()
        };
        let _ = namespace_api
            .create(&PostParams::default(), &namespace)
            .await;
        info!("Namespace is created successfully.");
    }

    worker::cleaner().await;

    Ok(())
}

pub async fn get_service(id: Uuid) -> Result<Service, ClusterError> {
    let service = get_services_by_label(&format!("cds/resource_id={}", id.to_string()))
        .await?
        .get(0)
        .ok_or(ClusterError::NotFound("service_not_found".to_owned()))?
        .to_owned();

    Ok(service)
}

pub async fn get_services_by_label(label: &str) -> Result<Vec<Service>, ClusterError> {
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
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
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let service = service_api.create(&Default::default(), &service).await?;

    Ok(service)
}

pub async fn delete_service(id: &str) -> Result<(), ClusterError> {
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let _ = service_api
        .delete_collection(
            &DeleteParams::default(),
            &ListParams::default().labels(&format!("cds/resource_id={id}")),
        )
        .await?;

    Ok(())
}

pub async fn get_pod(id: &str) -> Result<Pod, ClusterError> {
    let pod = get_pods_by_label(&format!("cds/resource_id={}", id.to_string()))
        .await?
        .get(0)
        .ok_or(ClusterError::NotFound("pod_not_found".to_owned()))?
        .to_owned();

    Ok(pod)
}

pub async fn get_pods_list() -> Result<Vec<Pod>, ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let pods = pod_api.list(&ListParams::default()).await?;

    Ok(pods.items)
}

pub async fn get_pods_by_label(label: &str) -> Result<Vec<Pod>, ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
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
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let pod = pod_api.create(&Default::default(), &pod).await?;

    Ok(pod)
}

pub async fn delete_pod(id: &str) -> Result<(), ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let _ = pod_api
        .delete_collection(
            &DeleteParams {
                grace_period_seconds: Some(0),
                ..Default::default()
            },
            &ListParams::default().labels(&format!("cds/resource_id={id}")),
        )
        .await?;

    Ok(())
}

pub async fn create_challenge_env(
    user: cds_db::transfer::User, team: Option<cds_db::transfer::Team>,
    game: Option<cds_db::transfer::Game>, challenge: cds_db::transfer::Challenge,
) -> Result<(), ClusterError> {
    let id = Uuid::new_v4();
    let name = format!("cds-{}", id.to_string());

    let mut desensitized_challenge = challenge.clone();
    desensitized_challenge.desensitize();

    let env = challenge.env.unwrap();

    let mut injected_flag = challenge.flags.clone().into_iter().next().unwrap();

    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])]").unwrap();
    if injected_flag.type_ == cds_db::entity::challenge::FlagType::Dynamic {
        injected_flag.value = re
            .replace_all(&injected_flag.value, Uuid::new_v4().simple().to_string())
            .to_string();
    }

    let metadata = ObjectMeta {
        name: Some(name.clone()),
        labels: Some(BTreeMap::from([
            ("cds/app".to_owned(), "challenges".to_owned()),
            ("cds/resource_id".to_owned(), id.to_string()),
            ("cds/user_id".to_owned(), format!("{}", user.id)),
            (
                "cds/team_id".to_owned(),
                format!("{}", match team {
                    Some(team) => team.id,
                    _ => 0,
                }),
            ),
            (
                "cds/game_id".to_owned(),
                format!("{}", match game {
                    Some(game) => game.id,
                    _ => 0,
                }),
            ),
            (
                "cds/challenge_id".to_owned(),
                format!("{}", challenge.id.to_string()),
            ),
        ])),
        annotations: Some(BTreeMap::from([
            (
                "cds/challenge".to_owned(),
                json!(desensitized_challenge).to_string(),
            ),
            ("cds/flag".to_owned(), format!("{}", injected_flag.value)),
            ("cds/renew".to_owned(), format!("{}", 0)),
            ("cds/duration".to_owned(), format!("{}", env.duration)),
            ("cds/ports".to_owned(), json!(env.ports).to_string()),
        ])),
        ..Default::default()
    };

    let mut env_vars: Vec<EnvVar> = env
        .envs
        .into_iter()
        .map(|env| EnvVar {
            name: env.0,
            value: Some(env.1),
            ..Default::default()
        })
        .collect();

    env_vars.push(EnvVar {
        name: injected_flag.env.unwrap_or("FLAG".to_owned()),
        value: Some(injected_flag.value),
        ..Default::default()
    });

    let container_ports: Vec<ContainerPort> = env
        .ports
        .iter()
        .map(|port| ContainerPort {
            container_port: *port,
            protocol: Some("TCP".to_owned()),
            ..Default::default()
        })
        .collect();

    let pod = Pod {
        metadata: metadata.clone(),
        spec: Some(PodSpec {
            containers: vec![K8sContainer {
                name: name.clone(),
                image: Some(env.image),
                env: Some(env_vars),
                ports: Some(container_ports),
                image_pull_policy: Some("IfNotPresent".to_owned()),
                resources: Some(ResourceRequirements {
                    requests: Some(
                        [("cpu", "10m".to_owned()), ("memory", "32Mi".to_owned())]
                            .iter()
                            .cloned()
                            .map(|(k, v)| (k.to_owned(), Quantity(v)))
                            .collect(),
                    ),
                    limits: Some(
                        [
                            ("cpu", env.cpu_limit.to_string()),
                            ("memory", format!("{}Mi", env.memory_limit)),
                        ]
                        .iter()
                        .cloned()
                        .map(|(k, v)| (k.to_owned(), Quantity(v)))
                        .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut pod = create_pod(pod).await?;

    let service_type = if cds_config::get_config().cluster.proxy.is_enabled {
        "ClusterIP"
    } else {
        "NodePort"
    };

    let service = Service {
        metadata: metadata.clone(),
        spec: Some(ServiceSpec {
            selector: Some(BTreeMap::from([(
                "cds/resource_id".to_owned(),
                id.to_string(),
            )])),
            ports: Some(
                env.ports
                    .iter()
                    .map(|port| ServicePort {
                        name: Some(port.to_string()),
                        port: *port,
                        target_port: None,
                        protocol: Some("TCP".to_owned()),
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

    let mut nats: BTreeMap<i32, i32> = BTreeMap::new();

    if let Some(spec) = service.spec {
        if let Some(ports) = spec.ports {
            for port in ports {
                if let Some(node_port) = port.node_port {
                    nats.insert(port.port, node_port);
                }
            }
        }
    }

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let annotations = pod.annotations_mut();
    annotations.insert(
        "cds/nats".to_owned(),
        nats.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(","),
    );

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
    let name = format!("cds-{}", id.to_string());
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );

    warn!("{}", name);

    let mut pod = get_pod(id).await?;

    let annotations = pod.annotations_mut();

    if let Some(renew) = annotations.get_mut("cds/renew") {
        *renew = format!("{}", renew.parse::<i64>().unwrap_or(0) + 1);
        warn!("{}", renew);
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

pub async fn wsrx(id: Uuid, port: u16, ws: WebSocket) -> Result<(), ClusterError> {
    let name = format!("cds-{}", id.to_string());

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );
    let mut pf = pod_api.portforward(&name, &[port]).await?;
    let pfw = pf.take_stream(port);
    if let Some(pfw) = pfw {
        let stream = Framed::new(pfw, wsrx::proxy::MessageCodec::new());
        let ws: wsrx::WrappedWsStream = ws.into();
        wsrx::proxy::proxy_stream(stream, ws).await?;
    }
    Ok(())
}

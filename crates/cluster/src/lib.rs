pub mod traits;

use std::{collections::BTreeMap, path::Path, process};
use std::fmt::format;
use axum::extract::ws::WebSocket;
use k8s_openapi::{
    api::core::v1::{
        Container as K8sContainer, ContainerPort, EnvVar, Namespace, Pod, PodSpec,
        ResourceRequirements, Service, ServicePort, ServiceSpec,
    },
    apimachinery::pkg::{api::resource::Quantity, apis::meta::v1::ObjectMeta},
};
use kube::{
    Client as K8sClient, Config,
    api::{Api, DeleteParams, ListParams, PostParams},
    config::Kubeconfig,
    runtime::wait::conditions,
};
use once_cell::sync::OnceCell;
use regex::Regex;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use sea_orm::ActiveValue::{Set, Unchanged};
use tokio_util::codec::Framed;
use tracing::{error, info};
use uuid::Uuid;
use cds_db::get_db;
use crate::traits::ClusterError;

static K8S_CLIENT: OnceCell<K8sClient> = OnceCell::new();

pub fn get_k8s_client() -> K8sClient {
    K8S_CLIENT.get().unwrap().clone()
}

pub async fn init() -> Result<(), ClusterError> {
    let result = Config::from_custom_kubeconfig(
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

    Ok(())
}

pub async fn create(id: Uuid) -> Result<(), ClusterError> {
    let name = format!("cds-{}",id.to_string());

    let pod = cds_db::entity::pod::Entity::find_by_id(id).one(get_db()).await.unwrap().ok_or("").unwrap();
    let challenge = cds_db::entity::challenge::Entity::find_by_id(pod.challenge_id).one(get_db()).await.unwrap().ok_or("").unwrap();

    let metadata = ObjectMeta {
        name: Some(name.clone()),
        labels: Some(BTreeMap::from([
            ("cds/app".to_owned(), "challenges".to_owned()),
            ("cds/resource_id".to_owned(), name.clone()),
        ])),
        ..Default::default()
    };

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );

    let env = challenge.env.unwrap();

    let mut env_vars: Vec<EnvVar> = env
        .envs
        .into_iter()
        .map(|env| EnvVar {
            name: env.0,
            value: Some(env.1),
            ..Default::default()
        })
        .collect();

    let mut injected_flag = challenge.flags.clone().into_iter().next().unwrap();

    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])]").unwrap();
    if injected_flag.type_ == cds_db::entity::challenge::FlagType::Dynamic {
        injected_flag.value = re
            .replace_all(&injected_flag.value, Uuid::new_v4().simple().to_string())
            .to_string();
    }

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

    let kube_pod = Pod {
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

    pod_api.create(&PostParams::default(), &kube_pod).await?;

    let mut nats: Vec<cds_db::entity::pod::Nat> = Vec::new();

    let service_type = if cds_config::get_config().cluster.proxy.is_enabled {
        "ClusterIP"
    } else {
        "NodePort"
    };
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );
    let service_ports: Vec<ServicePort> = env
        .ports
        .iter()
        .map(|port| ServicePort {
            name: Some(port.to_string()),
            port: *port,
            target_port: None,
            protocol: Some("TCP".to_owned()),
            ..Default::default()
        })
        .collect();

    let kube_service = Service {
        metadata: metadata.clone(),
        spec: Some(ServiceSpec {
            selector: Some(BTreeMap::from([(
                "cds/resource_id".to_owned(),
                name.clone(),
            )])),
            ports: Some(service_ports),
            type_: Some(service_type.to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    match service_api.create(&PostParams::default(), &kube_service).await {
        Ok(_) => {},
        Err(err) => {
            delete(id).await;
            return Err(ClusterError::KubeError(err));
        }
    };

    let kube_service = service_api.get(&name).await?;
    if let Some(spec) = kube_service.spec {
        if let Some(ports) = spec.ports {
            for port in ports {
                if let Some(node_port) = port.node_port {
                    nats.push(cds_db::entity::pod::Nat {
                        src: format!("{}", port.port),
                        dst: Some(format!("{}", node_port)),
                        proxy: cds_config::get_config().cluster.proxy.is_enabled,
                        entry: match cds_config::get_config().cluster.proxy.is_enabled {
                            true => Some("".to_owned()),
                            false => Some(format!(
                                "{}:{}",
                                cds_config::get_config().cluster.entry_host,
                                node_port
                            ))
                        },
                    });
                }
            }
        }
    }

    let _ = cds_db::entity::pod::ActiveModel {
        id: Unchanged(id),
        nats: Set(nats),
        ..Default::default()
    }.update(get_db()).await.unwrap();

    Ok(())
}

pub async fn delete(id: Uuid) {
    let name = format!("cds-{}", id.to_string());

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );
    let _ = pod_api.delete(&name, &DeleteParams::default()).await;
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_config::get_config().cluster.namespace.as_str(),
    );
    let _ = service_api.delete(&name, &DeleteParams::default()).await;
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

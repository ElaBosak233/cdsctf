pub mod traits;

use std::{collections::BTreeMap, process};
use std::path::Path;
use anyhow::anyhow;
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
    runtime::wait::conditions,
};
use kube::config::Kubeconfig;
use once_cell::sync::OnceCell;
use tokio_util::codec::Framed;
use tracing::{error, info};

use crate::traits::ClusterError;

static K8S_CLIENT: OnceCell<K8sClient> = OnceCell::new();

pub fn get_k8s_client() -> K8sClient {
    K8S_CLIENT.get().unwrap().clone()
}

pub async fn init() -> Result<(), ClusterError> {
    let result = Config::from_custom_kubeconfig(Kubeconfig::read_from(
        Path::new(cds_env::get_env().cluster.kube_config_path.as_str())
    )?, &Default::default()).await;
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
        namespace.metadata.name == Some(cds_env::get_env().clone().cluster.namespace)
    }) {
        let namespace = Namespace {
            metadata: ObjectMeta {
                name: Some(cds_env::get_env().clone().cluster.namespace),
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

pub async fn create(
    name: String, env: cds_db::entity::challenge::Env,
    injected_flag: cds_db::entity::challenge::Flag,
) -> Result<Vec<cds_db::entity::pod::Nat>, ClusterError> {
    let metadata = ObjectMeta {
        name: Some(name.clone()),
        labels: Some(BTreeMap::from([
            (String::from("cds/app"), String::from("challenges")),
            (String::from("cds/resource_id"), name.clone()),
        ])),
        ..Default::default()
    };

    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_env().cluster.namespace.as_str(),
    );

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
        name: injected_flag.env.unwrap_or("FLAG".to_string()),
        value: Some(injected_flag.value),
        ..Default::default()
    });

    let container_ports: Vec<ContainerPort> = env
        .ports
        .iter()
        .map(|port| ContainerPort {
            container_port: *port,
            protocol: Some("TCP".to_string()),
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
                image_pull_policy: Some(String::from("IfNotPresent")),
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

    pod_api.create(&PostParams::default(), &pod).await?;

    kube::runtime::wait::await_condition(pod_api.clone(), &name, conditions::is_pod_running())
        .await?;

    let mut nats: Vec<cds_db::entity::pod::Nat> = Vec::new();

    match cds_env::get_env().cluster.proxy.is_enabled {
        true => {
            for port in env.ports {
                nats.push(cds_db::entity::pod::Nat {
                    src: format!("{}", port),
                    dst: None,
                    proxy: cds_env::get_env().cluster.proxy.is_enabled,
                    entry: None,
                });
            }
        }
        false => {
            let service_api: Api<Service> = Api::namespaced(
                get_k8s_client(),
                cds_env::get_env().cluster.namespace.as_str(),
            );
            let service_ports: Vec<ServicePort> = env
                .ports
                .iter()
                .map(|port| ServicePort {
                    port: *port,
                    target_port: None,
                    protocol: Some("TCP".to_string()),
                    ..Default::default()
                })
                .collect();

            let service = Service {
                metadata: metadata.clone(),
                spec: Some(ServiceSpec {
                    selector: Some(BTreeMap::from([(
                        String::from("cds/resource_id"),
                        name.clone(),
                    )])),
                    ports: Some(service_ports),
                    type_: Some("NodePort".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            service_api.create(&PostParams::default(), &service).await?;

            let service = service_api.get(&name).await?;
            if let Some(spec) = service.spec {
                if let Some(ports) = spec.ports {
                    for port in ports {
                        if let Some(node_port) = port.node_port {
                            nats.push(cds_db::entity::pod::Nat {
                                src: format!("{}", port.port),
                                dst: Some(format!("{}", node_port)),
                                proxy: cds_env::get_env().cluster.proxy.is_enabled,
                                entry: Some(format!(
                                    "{}:{}",
                                    cds_config::get_config().await.cluster.entry,
                                    node_port
                                )),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(nats)
}

pub async fn delete(name: String) {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_env().cluster.namespace.as_str(),
    );
    let _ = pod_api.delete(&name, &DeleteParams::default()).await;
    let service_api: Api<Service> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_env().cluster.namespace.as_str(),
    );
    let _ = service_api.delete(&name, &DeleteParams::default()).await;
}

pub async fn wsrx(name: String, port: u16, ws: WebSocket) -> Result<(), ClusterError> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client(),
        cds_env::get_env().cluster.namespace.as_str(),
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

use std::{collections::BTreeMap, process, sync::OnceLock, time::Duration};

use axum::extract::ws::WebSocket;
use k8s_openapi::api::core::v1::{
    Container as K8sContainer, ContainerPort, EnvVar, Pod, PodSpec, Service, ServicePort,
    ServiceSpec,
};
use kube::{
    api::{Api, DeleteParams, ListParams, PostParams, ResourceExt},
    config::Kubeconfig,
    runtime::wait::conditions,
    Client as K8sClient, Config,
};
use tokio_util::codec::Framed;
use tracing::{error, info};

use crate::config;

static K8S_CLIENT: OnceLock<K8sClient> = OnceLock::new();

pub fn get_k8s_client() -> &'static K8sClient {
    K8S_CLIENT.get().unwrap()
}

pub async fn init() {
    match Config::from_kubeconfig(&Default::default()).await {
        Ok(config) => {
            let client = K8sClient::try_from(config).unwrap();
            let _ = K8S_CLIENT.set(client);
            info!("Kubernetes client initialized successfully.");
        }
        Err(e) => {
            error!(
                "Failed to create Kubernetes client from custom config: {:?}",
                e
            );
            process::exit(1);
        }
    }
}

pub async fn create(
    name: String, challenge: crate::model::challenge::Model,
    injected_flag: crate::model::challenge::Flag,
) -> Result<Vec<crate::model::pod::Nat>, anyhow::Error> {
    let client = get_k8s_client().clone();

    let metadata = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        name: Some(name.clone()),
        labels: Some(BTreeMap::from([(
            String::from("cds/resource_id"),
            name.clone(),
        )])),
        ..Default::default()
    };

    let pod_api: Api<Pod> = Api::namespaced(
        client.clone(),
        config::get_config().await.cluster.namespace.as_str(),
    );

    let mut env_vars: Vec<EnvVar> = challenge
        .envs
        .into_iter()
        .map(|env| EnvVar {
            name: env.key,
            value: Some(env.value),
            ..Default::default()
        })
        .collect();

    env_vars.push(EnvVar {
        name: injected_flag.env.unwrap_or("FLAG".to_string()),
        value: Some(injected_flag.value),
        ..Default::default()
    });

    let container_ports: Vec<ContainerPort> = challenge
        .ports
        .iter()
        .map(|port| ContainerPort {
            container_port: *port as i32,
            protocol: Some("TCP".to_string()),
            ..Default::default()
        })
        .collect();

    let pod = Pod {
        metadata: metadata.clone(),
        spec: Some(PodSpec {
            containers: vec![K8sContainer {
                name: name.clone(),
                image: challenge.image_name.clone(),
                env: Some(env_vars),
                ports: Some(match config::get_config().await.cluster.proxy.enabled {
                    true => vec![],
                    false => container_ports,
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

    let service_api: Api<Service> = Api::namespaced(
        client.clone(),
        config::get_config().await.cluster.namespace.as_str(),
    );

    let service = Service {
        metadata: metadata.clone(),
        spec: Some(ServiceSpec {
            selector: Some(BTreeMap::from([(
                String::from("cds/resource_id"),
                name.clone(),
            )])),
            ports: Some(
                challenge
                    .ports
                    .iter()
                    .map(|port| ServicePort {
                        port: *port as i32,
                        target_port: None,
                        protocol: Some("TCP".to_string()),
                        ..Default::default()
                    })
                    .collect(),
            ),
            type_: Some("NodePort".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    service_api.create(&PostParams::default(), &service).await?;

    let service = service_api.get(&name).await?;

    let mut nats: Vec<crate::model::pod::Nat> = Vec::new();
    if let Some(spec) = service.spec {
        if let Some(ports) = spec.ports {
            for port in ports {
                if let Some(node_port) = port.node_port {
                    nats.push(crate::model::pod::Nat {
                        src: format!("{}", port.port),
                        dst: Some(format!("{}", node_port)),
                        proxy: config::get_config().await.cluster.proxy.enabled,
                        entry: Some(format!(
                            "{}:{}",
                            config::get_config().await.cluster.entry,
                            node_port
                        )),
                    });
                }
            }
        }
    }

    Ok(nats)
}

pub async fn delete(name: String) {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client().clone(),
        config::get_config().await.cluster.namespace.as_str(),
    );
    let _ = pod_api.delete(&name, &DeleteParams::default()).await;
}

pub async fn wsrx(name: String, port: u16, ws: WebSocket) -> Result<(), anyhow::Error> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client().clone(),
        config::get_config().await.cluster.namespace.as_str(),
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

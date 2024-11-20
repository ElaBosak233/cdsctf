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
use once_cell::sync::OnceCell;
use tokio_util::codec::Framed;
use tracing::{error, info};

static K8S_CLIENT: OnceCell<K8sClient> = OnceCell::new();

pub fn get_k8s_client() -> &'static K8sClient {
    K8S_CLIENT.get().unwrap()
}

pub async fn init() {
    let result = Config::from_kubeconfig(&Default::default()).await;
    if let Err(e) = result {
        error!(
            "Failed to create Kubernetes client from custom config: {:?}",
            e
        );
        process::exit(1);
    }
    let config = result.unwrap();
    let client = K8sClient::try_from(config).unwrap();
    if let Err(_) = client.apiserver_version().await {
        error!("Failed to connect to Kubernetes API server.");
        process::exit(1);
    }
    let _ = K8S_CLIENT.set(client);
    info!("Kubernetes client initialized successfully.");
}

pub async fn create(
    name: String, challenge: crate::shared::Challenge,
    injected_flag: crate::db::entity::challenge::Flag,
) -> Result<Vec<crate::db::entity::pod::Nat>, anyhow::Error> {
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
        crate::env::get_env().cluster.namespace.as_str(),
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
                image: challenge.image_name.clone(),
                env: Some(env_vars),
                ports: Some(match crate::env::get_env().cluster.proxy.enabled {
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
        crate::env::get_env().cluster.namespace.as_str(),
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
                        port: *port,
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

    let mut nats: Vec<crate::db::entity::pod::Nat> = Vec::new();
    if let Some(spec) = service.spec {
        if let Some(ports) = spec.ports {
            for port in ports {
                if let Some(node_port) = port.node_port {
                    nats.push(crate::db::entity::pod::Nat {
                        src: format!("{}", port.port),
                        dst: Some(format!("{}", node_port)),
                        proxy: crate::env::get_env().cluster.proxy.enabled,
                        entry: Some(format!(
                            "{}:{}",
                            crate::config::get_config().await.cluster.entry,
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
        crate::env::get_env().cluster.namespace.as_str(),
    );
    let _ = pod_api.delete(&name, &DeleteParams::default()).await;
}

pub async fn wsrx(name: String, port: u16, ws: WebSocket) -> Result<(), anyhow::Error> {
    let pod_api: Api<Pod> = Api::namespaced(
        get_k8s_client().clone(),
        crate::env::get_env().cluster.namespace.as_str(),
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

use super::traits::Container;
use async_trait::async_trait;
use k8s_openapi::api::core::v1::{Container as K8sContainer, ContainerPort, EnvVar, Pod, PodSpec};
use kube::config::Kubeconfig;
use kube::runtime::wait::{await_condition, conditions};
use kube::{
    api::{Api, DeleteParams, ListParams, PostParams, ResourceExt},
    Client as K8sClient, Config,
};
use std::{error::Error, process, sync::OnceLock};
use tokio::time::Duration;
use tracing::{error, info};

static K8S_CLIENT: OnceLock<K8sClient> = OnceLock::new();

fn get_k8s_client() -> &'static K8sClient {
    return K8S_CLIENT.get().unwrap();
}

async fn daemon() {
    info!("Kubernetes container daemon has been started.");
    tokio::spawn(async {
        let interval = Duration::from_secs(10);
        loop {
            let pods: Api<Pod> = Api::namespaced(get_k8s_client().clone(), "default");
            let lp = ListParams::default().labels("expired=true");

            if let Ok(pod_list) = pods.list(&lp).await {
                for pod in pod_list {
                    let name = pod.name_any();
                    let _ = pods.delete(&name, &DeleteParams::default()).await;
                    info!("Cleaned up expired pod: {}", name);
                }
            }

            tokio::time::sleep(interval).await;
        }
    });
}

#[derive(Clone)]
pub struct K8s;

impl K8s {
    pub fn new() -> Self {
        return Self {};
    }
}

#[async_trait]
impl Container for K8s {
    async fn init(&self) {
        match Kubeconfig::read_from(crate::config::get_app_config().container.k8s.path.clone()) {
            Ok(config) => match Config::from_custom_kubeconfig(config, &Default::default()).await {
                Ok(config) => {
                    let client = K8sClient::try_from(config).unwrap();
                    let _ = K8S_CLIENT.set(client);
                    info!("Kubernetes client initialized successfully.");
                    daemon().await;
                }
                Err(e) => {
                    error!(
                        "Failed to create Kubernetes client from custom config: {:?}",
                        e
                    );
                    process::exit(1);
                }
            },
            Err(e) => {
                error!("Failed to read Kubernetes config file: {:?}", e);
                process::exit(1);
            }
        }
    }

    async fn create(
        &self, name: String, challenge: crate::model::challenge::Model,
        injected_flag: crate::model::challenge::Flag,
    ) -> Result<Vec<crate::model::pod::Nat>, Box<dyn Error>> {
        let client = get_k8s_client().clone();
        let pods: Api<Pod> = Api::namespaced(client, "default");

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
                container_port: port.value as i32,
                protocol: Some(port.protocol.to_uppercase()),
                ..Default::default()
            })
            .collect();

        let container = K8sContainer {
            name: name.clone(),
            image: challenge.image_name.clone(),
            env: Some(env_vars),
            ports: Some(container_ports),
            ..Default::default()
        };

        let pod_spec = PodSpec {
            containers: vec![container],
            ..Default::default()
        };

        let pod = Pod {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(name.clone()),
                ..Default::default()
            },
            spec: Some(pod_spec),
            ..Default::default()
        };

        pods.create(&PostParams::default(), &pod).await?;

        await_condition(pods.clone(), &name, conditions::is_pod_running()).await?;

        let pod = pods.get(&name).await?;
        let mut nats: Vec<crate::model::pod::Nat> = Vec::new();
        if let Some(status) = pod.status {
            if let Some(pod_ip) = status.pod_ip {
                for port in challenge.ports {
                    nats.push(crate::model::pod::Nat {
                        src: port.value.to_string(),
                        dst: pod_ip.clone(),
                        protocol: port.protocol.to_uppercase(),
                        ..Default::default()
                    });
                }
            }
        }

        return Ok(nats);
    }

    async fn delete(&self, name: String) {
        let pods: Api<Pod> = Api::namespaced(get_k8s_client().clone(), "default");
        let _ = pods.delete(&name, &DeleteParams::default()).await;
    }
}

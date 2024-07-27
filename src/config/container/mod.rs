pub mod docker;
pub mod k8s;
pub mod proxy;
pub mod strategy;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub entry: String,
    pub docker: docker::Config,
    pub k8s: k8s::Config,
    pub proxy: proxy::Config,
    pub strategy: strategy::Config,
}

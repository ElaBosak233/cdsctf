pub mod proxy;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub namespace: String,
    pub kube_config_path: String,
    pub proxy: proxy::Env,
}

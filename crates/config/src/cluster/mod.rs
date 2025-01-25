pub mod proxy;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub namespace: String,
    pub kube_config_path: String,
    pub proxy: proxy::Config,
    pub entry_host: String,
}

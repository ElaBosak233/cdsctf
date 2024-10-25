pub mod proxy;
pub mod strategy;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub entry: String,
    pub namespace: String,
    pub path: String,
    pub proxy: proxy::Config,
    pub strategy: strategy::Config,
}

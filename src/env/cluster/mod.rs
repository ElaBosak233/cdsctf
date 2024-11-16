pub mod proxy;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub namespace: String,
    pub proxy: proxy::Env,
}

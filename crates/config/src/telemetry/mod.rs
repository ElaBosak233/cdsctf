use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub is_enabled: bool,
    pub protocol: String,
    pub endpoint_url: String,
}

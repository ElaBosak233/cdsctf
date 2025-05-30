use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub is_enabled: bool,
    pub protocol: Protocol,
    pub endpoint_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Grpc,
    Json,
    Binary,
    #[serde(other)]
    Unknown,
}

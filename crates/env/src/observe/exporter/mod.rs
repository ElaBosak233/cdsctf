use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
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

impl Default for Config {
    fn default() -> Self {
        Self {
            is_enabled: false,
            protocol: Protocol::Grpc,
            endpoint_url: "http://otel:4317".to_string(),
        }
    }
}

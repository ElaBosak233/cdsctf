use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub is_enabled: bool,
    pub host: String,
    pub port: u16,
    pub tls: Tls,
    pub username: String,
    pub password: String,
    pub whitelist: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Tls {
    Starttls,
    Tls,
    #[default]
    #[serde(other)]
    None,
}

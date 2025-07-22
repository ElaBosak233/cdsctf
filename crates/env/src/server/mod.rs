use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend: String,
    pub burst_restore_rate: u64,
    pub burst_limit: u32,
    pub cors_origins: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "0.0.0.0".to_owned(),
            port: 8888,
            frontend: "./dist".to_owned(),
            burst_restore_rate: 100,
            burst_limit: 512,
            cors_origins: "*".to_owned(),
        }
    }
}

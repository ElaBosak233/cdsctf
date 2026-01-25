mod rate_limit;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend: String,
    pub rate_limit: rate_limit::Config,
    pub cors_origins: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "0.0.0.0".to_owned(),
            port: 8888,
            frontend: "./dist".to_owned(),
            rate_limit: rate_limit::Config::default(),
            cors_origins: "*".to_owned(),
        }
    }
}

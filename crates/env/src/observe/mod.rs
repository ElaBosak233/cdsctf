pub mod exporter;
pub mod logger;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub service_name: String,
    pub logger: logger::Config,
    pub exporter: exporter::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            service_name: "cdsctf".to_string(),
            logger: logger::Config::default(),
            exporter: exporter::Config::default(),
        }
    }
}

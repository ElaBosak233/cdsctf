use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub metric_endpoint: Option<String>,
    pub log_endpoint: Option<String>,
    pub trace_endpoint: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: None,
            metric_endpoint: None,
            log_endpoint: None,
            trace_endpoint: None,
        }
    }
}

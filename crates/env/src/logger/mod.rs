use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

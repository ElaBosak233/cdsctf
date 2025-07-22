use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: "./data/media".to_string(),
        }
    }
}

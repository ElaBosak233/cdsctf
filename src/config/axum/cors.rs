use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub allow_origins: Vec<String>,
    pub allow_methods: Vec<String>,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub namespace: String,
    pub auto_infer: bool,
    pub config_path: String,
    pub traffic: Traffic,
    pub public_entries: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Traffic {
    Expose,
    Proxy,
}

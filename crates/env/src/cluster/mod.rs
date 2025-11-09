use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub namespace: String,
    pub auto_infer: bool,
    pub config_path: String,
    pub traffic: Traffic,
    pub public_entry: String,
    pub egress_excluded_cidrs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Traffic {
    Expose,
    Proxy,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            namespace: "cdsctf-challenges".to_owned(),
            auto_infer: true,
            config_path: "".to_owned(),
            traffic: Traffic::Proxy,
            public_entry: "0.0.0.0".to_owned(),
            egress_excluded_cidrs: vec![],
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub prefix: String,
    pub path_style: bool,
    pub presigned: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: "s3://media:9000".to_string(),
            region: "us-east-1".to_string(),
            bucket: "cdsctf".to_string(),
            access_key: "rustfsadmin".to_string(),
            secret_key: "rustfsadmin".to_string(),
            prefix: String::new(),
            path_style: true,
            presigned: false,
        }
    }
}

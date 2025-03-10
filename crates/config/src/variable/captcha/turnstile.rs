use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub secret_key: String,
    pub site_key: String,
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub url: String,
    pub secret_key: String,
    pub site_key: String,
}

impl Config {
    pub fn desensitize(&self) -> Self {
        Self {
            secret_key: "".to_owned(),
            ..self.to_owned()
        }
    }
}

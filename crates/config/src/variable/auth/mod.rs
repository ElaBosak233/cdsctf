use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub secret: String,
    pub expiration: i64,
    pub is_registration_enabled: bool,
}

impl Config {
    pub fn desensitize(&self) -> Self {
        Self {
            secret: "".to_string(),
            ..self.to_owned()
        }
    }
}

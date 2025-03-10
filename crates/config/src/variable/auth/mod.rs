use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub secret: String,
    pub expiration: i64,
    pub is_registration_enabled: bool,
}

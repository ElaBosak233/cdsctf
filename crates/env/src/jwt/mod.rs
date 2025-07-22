use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub secret: String,
    pub expiration: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            secret: nanoid!(32),
            expiration: 43200,
        }
    }
}

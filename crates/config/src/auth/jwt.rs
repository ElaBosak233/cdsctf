use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Config {
    pub secret_key: String,
    pub expiration: i64,
}

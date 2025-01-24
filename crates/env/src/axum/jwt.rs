use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub secret: String,
    pub expiration: i64,
}

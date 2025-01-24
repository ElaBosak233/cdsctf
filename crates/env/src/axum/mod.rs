pub mod jwt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub host: String,
    pub port: u16,
    pub jwt: jwt::Env,
}

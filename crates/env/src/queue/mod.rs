use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub token: String,
    pub tls: bool,
}

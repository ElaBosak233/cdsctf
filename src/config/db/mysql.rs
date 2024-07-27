use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub dbname: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub token: String,
    pub tls: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "queue".to_owned(),
            port: 4222,
            username: "".to_owned(),
            password: "".to_owned(),
            token: "".to_owned(),
            tls: false,
        }
    }
}

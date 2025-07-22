use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "db".to_owned(),
            port: 5432,
            dbname: "cdsctf".to_owned(),
            username: "cdsctf".to_owned(),
            password: "cdsctf".to_owned(),
            ssl_mode: "disable".to_owned(),
        }
    }
}

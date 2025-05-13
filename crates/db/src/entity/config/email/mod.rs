use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq)]
pub struct Config {
    pub is_enabled: bool,
    pub host: String,
    pub port: u16,
    pub tls: Tls,
    pub username: String,
    pub password: String,
    pub whitelist: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct Mail {
    pub subject: String,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Tls {
    Starttls,
    Tls,
    #[default]
    #[serde(other)]
    None,
}

impl Config {
    pub fn desensitize(&self) -> Self {
        Self {
            username: "".to_owned(),
            password: "".to_owned(),
            host: "".to_owned(),
            port: 0,
            tls: Tls::None,
            ..self.to_owned()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_enabled: false,
            host: "".to_owned(),
            port: 0,
            tls: Tls::None,
            username: "".to_owned(),
            password: "".to_owned(),
            whitelist: vec![],
        }
    }
}

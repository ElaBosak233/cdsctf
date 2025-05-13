use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq)]
pub struct Config {
    pub url: String,
    pub secret_key: String,
    pub site_key: String,
}

impl Config {
    pub fn desensitize(&self) -> Self {
        Self {
            secret_key: "".to_owned(),
            ..self.to_owned()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "https://hcaptcha.com/siteverify".to_owned(),
            secret_key: "".to_owned(),
            site_key: "".to_owned(),
        }
    }
}

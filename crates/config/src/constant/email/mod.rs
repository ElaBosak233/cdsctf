use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub is_enabled: bool,
    pub host: String,
    pub port: u16,
    pub tls: Tls,
    pub username: String,
    pub password: String,
    pub whitelist: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_password: Option<Mail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_email: Option<Mail>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Mail {
    pub subject: String,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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
            reset_password: None,
            verify_email: None,
            ..self.to_owned()
        }
    }
}

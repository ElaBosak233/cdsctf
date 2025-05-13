pub mod hcaptcha;
pub mod turnstile;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq)]
pub struct Config {
    pub provider: Provider,
    pub difficulty: u64,
    pub turnstile: turnstile::Config,
    pub hcaptcha: hcaptcha::Config,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Pow,
    Image,
    Turnstile,
    #[serde(rename = "hcaptcha")]
    HCaptcha,
    #[default]
    #[serde(other)]
    None,
}

impl Config {
    pub fn desensitize(&self) -> Self {
        Self {
            turnstile: self.turnstile.desensitize(),
            hcaptcha: self.hcaptcha.desensitize(),
            ..self.to_owned()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: Provider::Pow,
            difficulty: 2,
            hcaptcha: hcaptcha::Config::default(),
            turnstile: turnstile::Config::default(),
        }
    }
}

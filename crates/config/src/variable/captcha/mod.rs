pub mod hcaptcha;
pub mod turnstile;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub provider: Provider,
    pub difficulty: u64,
    pub turnstile: turnstile::Config,
    pub hcaptcha: hcaptcha::Config,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

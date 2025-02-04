pub mod hcaptcha;
pub mod turnstile;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: Provider,
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

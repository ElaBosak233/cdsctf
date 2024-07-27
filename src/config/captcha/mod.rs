pub mod recaptcha;
pub mod turnstile;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub turnstile: turnstile::Config,
    pub recaptcha: recaptcha::Config,
}

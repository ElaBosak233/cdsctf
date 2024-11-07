use recaptcha::Recaptcha;
use tracing::error;
use traits::Captcha;
use turnstile::Turnstile;

use crate::config;

pub mod recaptcha;
pub mod traits;
pub mod turnstile;

pub fn new() -> Option<Box<dyn Captcha + Send + Sync>> {
    match config::get_config()
        .captcha
        .provider
        .to_lowercase()
        .as_str()
    {
        "recaptcha" => Some(Box::new(Recaptcha::new())),
        "turnstile" => Some(Box::new(Turnstile::new())),
        _ => {
            error!("Invalid captcha provider");
            None
        }
    }
}

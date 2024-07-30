use std::sync::Arc;

use recaptcha::Recaptcha;
use tracing::error;
use traits::Captcha;
use turnstile::Turnstile;

use crate::config;

pub mod recaptcha;
pub mod traits;
pub mod turnstile;

pub fn new() -> Option<Box<dyn Captcha>> {
    match config::get_app_config()
        .captcha
        .provider
        .to_lowercase()
        .as_str()
    {
        "recaptcha" => return Some(Box::new(Recaptcha::new())),
        "turnstile" => return Some(Box::new(Turnstile::new())),
        _ => {
            error!("Invalid captcha provider");
            return None;
        }
    }
}

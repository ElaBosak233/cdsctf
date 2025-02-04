use cds_config::captcha::Provider;

use crate::traits::CaptchaError;
pub use crate::traits::{Answer, Captcha};

mod hcaptcha;
mod pow;
pub mod traits;
mod turnstile;

pub async fn init() -> Result<(), CaptchaError> {
    Ok(())
}

pub async fn generate() -> Result<Option<Captcha>, CaptchaError> {
    match cds_config::get_config().captcha.provider {
        _ => Ok(None),
    }
}

pub async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    match cds_config::get_config().captcha.provider {
        Provider::Turnstile => turnstile::check(answer).await,
        Provider::HCaptcha => hcaptcha::check(answer).await,
        _ => Ok(true),
    }
}

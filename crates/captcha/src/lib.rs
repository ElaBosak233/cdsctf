mod hcaptcha;
mod image;
mod pow;
pub mod traits;
mod turnstile;

use cds_db::config::captcha::Provider;

use crate::traits::CaptchaError;
pub use crate::traits::{Answer, Captcha};

pub async fn init() -> Result<(), CaptchaError> {
    Ok(())
}

pub async fn generate() -> Result<Option<Captcha>, CaptchaError> {
    match cds_db::get_config().await.captcha.provider {
        Provider::Pow => Ok(Some(pow::generate().await?)),
        Provider::Image => Ok(Some(image::generate().await?)),
        _ => Ok(None),
    }
}

pub async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    match cds_db::get_config().await.captcha.provider {
        Provider::Pow => pow::check(answer).await,
        Provider::Image => image::check(answer).await,
        Provider::Turnstile => turnstile::check(answer).await,
        Provider::HCaptcha => hcaptcha::check(answer).await,
        _ => Ok(true),
    }
}

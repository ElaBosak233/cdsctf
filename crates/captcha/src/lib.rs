mod hcaptcha;
mod image;
mod pow;
pub mod traits;
mod turnstile;

use cds_cache::Cache;
use cds_db::{DB, config::captcha::Provider};

use crate::traits::CaptchaError;
pub use crate::traits::{Answer, CaptchaChallenge};

#[derive(Clone)]
pub struct Captcha {
    db: DB,
    cache: Cache,
}

pub fn init(db: &DB, cache: &Cache) -> Result<Captcha, CaptchaError> {
    Ok(Captcha {
        db: db.clone(),
        cache: cache.clone(),
    })
}

impl Captcha {
    pub async fn generate(&self) -> Result<Option<CaptchaChallenge>, CaptchaError> {
        match cds_db::get_config(&self.db.conn).await.captcha.provider {
            Provider::Pow => Ok(Some(pow::generate(self).await?)),
            Provider::Image => Ok(Some(image::generate(self).await?)),
            _ => Ok(None),
        }
    }

    pub async fn check(&self, answer: &Answer) -> Result<bool, CaptchaError> {
        match cds_db::get_config(&self.db.conn).await.captcha.provider {
            Provider::Pow => pow::check(self, answer).await,
            Provider::Image => image::check(self, answer).await,
            Provider::Turnstile => turnstile::check(self, answer).await,
            Provider::HCaptcha => hcaptcha::check(self, answer).await,
            _ => Ok(true),
        }
    }
}

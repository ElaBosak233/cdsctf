//! Captcha abstraction: provider is selected from database configuration.
//!
//! Supported backends include proof-of-work, generated images, Cloudflare
//! Turnstile, and hCaptcha. Some providers return `Ok(None)` on generate when
//! not applicable.

/// Defines the `hcaptcha` submodule (see sibling `*.rs` files).
mod hcaptcha;

/// Defines the `image` submodule (see sibling `*.rs` files).
mod image;

/// Defines the `pow` submodule (see sibling `*.rs` files).
mod pow;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Defines the `turnstile` submodule (see sibling `*.rs` files).
mod turnstile;

use cds_cache::Cache;
use cds_db::{DB, config::captcha::Provider};

use crate::traits::CaptchaError;
pub use crate::traits::{Answer, CaptchaChallenge};

/// Holds DB + cache handles needed by individual provider implementations.
#[derive(Clone)]
pub struct Captcha {
    db: DB,
    cache: Cache,
}

/// Constructs a captcha service sharing the same database and Redis clients as
/// the app.
pub fn init(db: &DB, cache: &Cache) -> Result<Captcha, CaptchaError> {
    Ok(Captcha {
        db: db.clone(),
        cache: cache.clone(),
    })
}

impl Captcha {
    /// Builds a challenge object for the active provider, or `None` when the
    /// provider does not need a server-side challenge.
    pub async fn generate(&self) -> Result<Option<CaptchaChallenge>, CaptchaError> {
        match cds_db::get_config(&self.db.conn).await.captcha.provider {
            Provider::Pow => Ok(Some(pow::generate(self).await?)),
            Provider::Image => Ok(Some(image::generate(self).await?)),
            _ => Ok(None),
        }
    }

    /// Validates the user-submitted [`Answer`] against the configured provider
    /// (always `true` for disabled/none providers).
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

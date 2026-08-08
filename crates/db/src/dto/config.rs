use serde::{Deserialize, Serialize};

use crate::entity::config::{Config, auth, captcha, meta};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PublicConfig {
    pub meta: meta::Config,
    pub auth: auth::Config,
    pub email: PublicEmailConfig,
    pub captcha: PublicCaptchaConfig,
    pub logo_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PublicEmailConfig {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PublicCaptchaConfig {
    pub provider: captcha::Provider,
    pub difficulty: u64,
    pub turnstile: PublicCaptchaSiteConfig,
    pub hcaptcha: PublicCaptchaSiteConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PublicCaptchaSiteConfig {
    pub site_key: String,
}

impl From<&Config> for PublicConfig {
    fn from(config: &Config) -> Self {
        Self {
            meta: config.meta.clone(),
            auth: config.auth.clone(),
            email: PublicEmailConfig {
                enabled: config.email.enabled,
            },
            captcha: PublicCaptchaConfig {
                provider: config.captcha.provider.clone(),
                difficulty: config.captcha.difficulty,
                turnstile: PublicCaptchaSiteConfig {
                    site_key: config.captcha.turnstile.site_key.clone(),
                },
                hcaptcha: PublicCaptchaSiteConfig {
                    site_key: config.captcha.hcaptcha.site_key.clone(),
                },
            },
            logo_hash: config.logo_hash.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PublicConfig;
    use crate::entity::config::Config;

    #[test]
    fn public_config_json_excludes_service_credentials() {
        let mut config = Config::default();
        config.email.host = "smtp.internal".to_owned();
        config.email.username = "mailer".to_owned();
        config.email.password = "secret".to_owned();
        config.email.whitelist = vec!["internal.example".to_owned()];
        config.captcha.turnstile.secret_key = "turnstile-secret".to_owned();
        config.captcha.hcaptcha.secret_key = "hcaptcha-secret".to_owned();

        let value = serde_json::to_value(PublicConfig::from(&config)).unwrap();
        assert_eq!(value["email"], serde_json::json!({"enabled": false}));
        assert!(value["captcha"]["turnstile"].get("secret_key").is_none());
        assert!(value["captcha"]["turnstile"].get("url").is_none());
        assert!(value["captcha"]["hcaptcha"].get("secret_key").is_none());
    }
}

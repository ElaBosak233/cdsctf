//! SeaORM `mod` entity — maps the `mod` table and its relations.

/// Defines the `turnstile` submodule (see sibling `*.rs` files).
pub mod turnstile;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq, utoipa::ToSchema,
)]
pub struct Config {
    pub provider: Provider,
    pub difficulty: u64,
    pub turnstile: turnstile::Config,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Pow,
    Image,
    Turnstile,
    // Preserve recognition of old persisted values without enabling the removed provider.
    #[serde(rename = "hcaptcha")]
    LegacyHCaptcha,
    #[default]
    #[serde(other)]
    None,
}

impl Config {
    /// Strips secrets so configuration can be returned to clients.
    pub fn desensitize(&self) -> Self {
        Self {
            turnstile: self.turnstile.desensitize(),
            ..self.to_owned()
        }
    }
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Self {
            provider: Provider::Pow,
            difficulty: 2,
            turnstile: turnstile::Config::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Provider};

    #[test]
    fn legacy_hcaptcha_config_remains_readable_but_disabled() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "provider": "hcaptcha",
            "difficulty": 2,
            "turnstile": {
                "url": "https://example.com/siteverify",
                "secret_key": "",
                "site_key": ""
            },
            "hcaptcha": {
                "url": "https://hcaptcha.com/siteverify",
                "secret_key": "legacy-secret",
                "site_key": "legacy-site-key"
            }
        }))
        .unwrap();

        assert_eq!(config.provider, Provider::LegacyHCaptcha);
    }
}

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

//! SeaORM `mod` entity — maps the `mod` table and its relations.

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq, utoipa::ToSchema,
)]
pub struct Config {
    pub registration_enabled: bool,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Self {
            registration_enabled: true,
        }
    }
}

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq, utoipa::ToSchema,
)]
pub struct Config {
    pub registration_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            registration_enabled: true,
        }
    }
}

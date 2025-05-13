use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq)]
pub struct Config {
    pub is_registration_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_registration_enabled: true,
        }
    }
}

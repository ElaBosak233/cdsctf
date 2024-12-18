pub mod jwt;
pub mod registration;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct Config {
    pub jwt: jwt::Config,
    pub registration: registration::Config,
}

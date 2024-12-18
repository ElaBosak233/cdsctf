pub mod proxy;
pub mod strategy;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct Config {
    pub entry: String,
    pub strategy: strategy::Config,
}

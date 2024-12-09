use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, PartialEq, Eq, Default)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub color: String,
    pub favicon: String,
}

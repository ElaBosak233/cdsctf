use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq)]
pub struct Config {
    pub title: String,
    pub description: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "CdsCTF".to_string(),
            description: "Stay determined".to_string(),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
    pub id: i64,
    pub value: serde_json::Value,
}

impl From<entity::config::Model> for Config {
    fn from(model: entity::config::Model) -> Self {
        Self {
            id: model.id,
            value: model.value,
        }
    }
}

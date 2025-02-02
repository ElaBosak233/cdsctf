use std::str::FromStr;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameNotice {
    pub id: i64,
    pub game_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}

impl From<entity::game_notice::Model> for GameNotice {
    fn from(entity: entity::game_notice::Model) -> Self {
        Self {
            id: entity.id,
            game_id: entity.game_id,
            title: entity.title,
            content: entity.content,
            created_at: entity.created_at,
        }
    }
}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameTeamUser {
    pub user_id: i64,
    pub game_team_id: i64,
}

impl From<entity::team_user::Model> for GameTeamUser {
    fn from(entity: entity::team_user::Model) -> Self {
        Self {
            user_id: entity.user_id,
            game_team_id: entity.team_id,
        }
    }
}

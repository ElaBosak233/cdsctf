use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};

use super::ActiveModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
}

impl Default for FindRequest {
    fn default() -> Self {
        FindRequest { game_id: None, team_id: None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub game_id: i64,
    pub team_id: i64,
}

impl From<CreateRequest> for ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            game_id: Set(req.game_id),
            team_id: Set(req.team_id),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_allowed: Option<bool>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            game_id: req.game_id.map_or(NotSet, |v| Set(v)),
            team_id: req.team_id.map_or(NotSet, |v| Set(v)),
            is_allowed: req.is_allowed.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

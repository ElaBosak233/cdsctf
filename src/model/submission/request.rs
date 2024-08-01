use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{ActiveModel, Status};

#[derive(Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub flag: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
}

impl From<CreateRequest> for super::ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            flag: Set(req.flag),
            user_id: req.user_id.map_or(NotSet, |v| Set(v)),
            team_id: req.team_id.map_or(NotSet, |v| Set(Some(v))),
            game_id: req.game_id.map_or(NotSet, |v| Set(Some(v))),
            challenge_id: req.challenge_id.map_or(NotSet, |v| Set(v)),
            status: Set(Status::Pending),
            rank: Set(0),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub flag: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub rank: Option<i64>,
    pub status: Option<Status>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            id: req.id.map_or(NotSet, |v| Set(v)),
            flag: req.flag.map_or(NotSet, |v| Set(v)),
            user_id: req.user_id.map_or(NotSet, |v| Set(v)),
            team_id: req.team_id.map_or(NotSet, |v| Set(Some(v))),
            game_id: req.game_id.map_or(NotSet, |v| Set(Some(v))),
            challenge_id: req.challenge_id.map_or(NotSet, |v| Set(v)),
            rank: req.rank.map_or(NotSet, |v| Set(v)),
            status: req.status.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

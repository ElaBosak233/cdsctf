use serde::{Deserialize, Serialize};

use crate::model::{challenge, team, user};

use super::Status;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSubmissionModel {
    pub id: i64,
    pub flag: String,
    pub status: Status,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub pts: i64,
    pub rank: i64,
    pub user: Option<user::Model>,
    pub team: Option<team::Model>,
    pub challenge: Option<challenge::Model>,
}

impl From<super::Model> for GameSubmissionModel {
    fn from(model: super::Model) -> Self {
        return Self {
            id: model.id,
            flag: model.flag,
            status: model.status,
            user_id: model.user_id,
            team_id: model.team_id,
            game_id: model.game_id,
            challenge_id: model.challenge_id,
            created_at: model.created_at,
            updated_at: model.updated_at,

            pts: 0,
            rank: 0,

            user: model.user,
            team: model.team,
            challenge: model.challenge,
        };
    }
}

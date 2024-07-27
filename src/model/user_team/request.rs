use sea_orm::Set;
use serde::{Deserialize, Serialize};

use super::ActiveModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinRequest {
    pub user_id: i64,
    pub team_id: i64,
    pub invite_token: String,
}

impl From<JoinRequest> for ActiveModel {
    fn from(req: JoinRequest) -> Self {
        Self {
            user_id: Set(req.user_id),
            team_id: Set(req.team_id),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub user_id: i64,
    pub team_id: i64,
}

impl From<CreateRequest> for ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            user_id: Set(req.user_id),
            team_id: Set(req.team_id),
            ..Default::default()
        }
    }
}

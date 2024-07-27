use sea_orm::Set;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub is_available: Option<bool>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Default for FindRequest {
    fn default() -> Self {
        FindRequest {
            id: None,
            name: None,
            user_id: None,
            team_id: None,
            game_id: None,
            challenge_id: None,
            is_available: None,
            is_detailed: Some(false),
            page: None,
            size: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub challenge_id: i64,
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
}

impl From<CreateRequest> for super::ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            challenge_id: Set(req.challenge_id),
            ..Default::default()
        }
    }
}

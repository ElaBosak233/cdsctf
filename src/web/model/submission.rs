use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<crate::model::submission::Status>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub code: u16,
    pub data: Vec<crate::model::submission::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetByIDResponse {
    pub code: u16,
    pub data: crate::model::submission::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub flag: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub code: u16,
    pub data: crate::model::submission::Model,
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
    pub status: Option<crate::model::submission::Status>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteResponse {
    pub code: u16,
}

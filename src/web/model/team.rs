use serde::{Deserialize, Serialize};
use validator::Validate;

use super::Metadata;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<i64>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub code: u16,
    pub data: Vec<crate::model::team::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub name: String,
    pub email: String,
    pub captain_id: i64,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub code: u16,
    pub data: crate::model::team::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub captain_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub code: u16,
    pub data: crate::model::team::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub user_id: i64,
    pub team_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinRequest {
    pub user_id: i64,
    pub team_id: i64,
    pub invite_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinResponse {
    pub code: u16,
    pub data: crate::model::user_team::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetInviteTokenResponse {
    pub code: u16,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInviteTokenResponse {
    pub code: u16,
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAvatarMetadataResponse {
    pub code: u16,
    pub data: Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveAvatarResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAvatarResponse {
    pub code: u16,
}

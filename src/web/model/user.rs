use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::user::group::Group;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub code: u16,
    pub data: Vec<crate::model::user::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub group: Group,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub code: u16,
    pub data: crate::model::user::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub code: u16,
    pub data: crate::model::user::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamResponse {
    pub code: u16,
    pub data: Vec<crate::model::team::Model>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub code: u16,
    pub data: crate::model::user::Model,
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub code: u16,
    pub data: crate::model::user::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAvatarMetadataResponse {
    pub code: u16,
    pub data: super::Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveAvatarResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAvatarResponse {
    pub code: u16,
}

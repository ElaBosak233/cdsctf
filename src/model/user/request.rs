use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::ActiveModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Default for FindRequest {
    fn default() -> Self {
        FindRequest {
            id: None,
            name: None,
            email: None,
            group: None,
            page: None,
            size: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub group: String,
}

impl From<CreateRequest> for ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            username: Set(req.username),
            nickname: Set(req.nickname),
            email: Set(req.email),
            password: Set(req.password),
            group: Set(req.group),
            ..Default::default()
        }
    }
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
    pub group: Option<String>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            id: req.id.map_or(NotSet, |v| Set(v)),
            username: req.username.map_or(NotSet, |v| Set(v)),
            nickname: req.nickname.map_or(NotSet, |v| Set(v)),
            email: req.email.map_or(NotSet, |v| Set(v)),
            password: req.password.map_or(NotSet, |v| Set(v)),
            group: req.group.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
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

impl From<RegisterRequest> for ActiveModel {
    fn from(req: RegisterRequest) -> Self {
        Self {
            username: Set(req.username),
            nickname: Set(req.nickname),
            email: Set(req.email),
            password: Set(req.password),
            ..Default::default()
        }
    }
}

use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::ActiveModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Default for FindRequest {
    fn default() -> Self {
        FindRequest {
            id: None,
            name: None,
            email: None,
            page: None,
            size: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub name: String,
    pub email: String,
    pub captain_id: i64,
    pub description: Option<String>,
}

impl From<CreateRequest> for ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            name: Set(req.name),
            email: Set(Some(req.email)),
            description: req.description.map_or(NotSet, |v| Set(Some(v))),
            captain_id: Set(req.captain_id),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub captain_id: Option<i64>,
    pub description: Option<String>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            id: req.id.map_or(NotSet, |v| Set(v)),
            name: req.name.map_or(NotSet, |v| Set(v)),
            email: req.email.map_or(NotSet, |v| Set(Some(v))),
            captain_id: req.captain_id.map_or(NotSet, |v| Set(v)),
            description: req.description.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        }
    }
}

use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::ActiveModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateRequest {
    pub name: String,
    pub color: String,
    pub icon: String,
}

impl From<CreateRequest> for super::ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            name: Set(req.name),
            color: Set(req.color),
            icon: Set(req.icon),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            id: req.id.map_or(NotSet, |v| Set(v)),
            name: req.name.map_or(NotSet, |v| Set(v)),
            color: req.color.map_or(NotSet, |v| Set(v)),
            icon: req.icon.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

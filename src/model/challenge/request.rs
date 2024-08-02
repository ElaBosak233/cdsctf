use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::ActiveModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category_id: Option<i64>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub is_detailed: Option<bool>,
    pub user_id: Option<i64>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub description: String,
    pub category_id: i64,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub difficulty: Option<i64>,
    pub image_name: Option<String>,
    pub cpu_limit: Option<i64>,
    pub memory_limit: Option<i64>,
    pub duration: Option<i64>,
    pub ports: Option<super::Ports>,
    pub envs: Option<Vec<super::Env>>,
    pub flags: Option<Vec<super::Flag>>,
}

impl From<CreateRequest> for super::ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            title: Set(req.title),
            description: Set(Some(req.description)),
            category_id: Set(req.category_id),
            is_practicable: Set(req.is_practicable.unwrap_or(false)),
            is_dynamic: Set(req.is_dynamic.unwrap_or(false)),
            has_attachment: Set(req.has_attachment.unwrap_or(false)),
            image_name: Set(req.image_name),
            cpu_limit: Set(req.cpu_limit.unwrap_or(0)),
            memory_limit: Set(req.memory_limit.unwrap_or(0)),
            duration: Set(req.duration.unwrap_or(1800)),
            ports: Set(req.ports.unwrap_or(super::Ports::default())),
            envs: Set(req.envs.unwrap_or(vec![])),
            flags: Set(req.flags.unwrap_or(vec![])),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub difficulty: Option<i64>,
    pub image_name: Option<String>,
    pub cpu_limit: Option<i64>,
    pub memory_limit: Option<i64>,
    pub duration: Option<i64>,
    pub ports: Option<super::Ports>,
    pub envs: Option<Vec<super::Env>>,
    pub flags: Option<Vec<super::Flag>>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            id: req.id.map_or(NotSet, |v| Set(v)),
            title: req.title.map_or(NotSet, |v| Set(v)),
            description: req.description.map_or(NotSet, |v| Set(Some(v))),
            category_id: req.category_id.map_or(NotSet, |v| Set(v)),
            is_practicable: req.is_practicable.map_or(NotSet, |v| Set(v)),
            is_dynamic: req.is_dynamic.map_or(NotSet, |v| Set(v)),
            has_attachment: req.has_attachment.map_or(NotSet, |v| Set(v)),
            image_name: req.image_name.map_or(NotSet, |v| Set(Some(v))),
            cpu_limit: req.cpu_limit.map_or(NotSet, |v| Set(v)),
            memory_limit: req.memory_limit.map_or(NotSet, |v| Set(v)),
            duration: req.duration.map_or(NotSet, |v| Set(v)),
            ports: req.ports.map_or(NotSet, |v| Set(v)),
            envs: req.envs.map_or(NotSet, |v| Set(v)),
            flags: req.flags.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusRequest {
    pub cids: Vec<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

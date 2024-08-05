use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::ActiveModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Default for FindRequest {
    fn default() -> Self {
        FindRequest {
            id: None,
            title: None,
            is_enabled: None,
            page: None,
            size: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub title: String,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

impl From<CreateRequest> for ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            title: Set(req.title),
            bio: Set(req.bio),
            description: Set(req.description),
            is_enabled: Set(req.is_enabled.unwrap_or(false)),
            is_public: Set(req.is_public.unwrap_or(false)),

            member_limit_min: req.member_limit_min.map_or(NotSet, |v| Set(v)),
            member_limit_max: req.member_limit_max.map_or(NotSet, |v| Set(v)),
            parallel_container_limit: req.parallel_container_limit.map_or(NotSet, |v| Set(v)),

            is_need_write_up: Set(req.is_need_write_up.unwrap_or(false)),
            started_at: req.started_at.map_or(NotSet, |v| Set(v)),
            ended_at: req.ended_at.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            id: req.id.map_or(NotSet, |v| Set(v)),
            title: req.title.map_or(NotSet, |v| Set(v)),
            bio: req.bio.map_or(NotSet, |v| Set(Some(v))),
            description: req.description.map_or(NotSet, |v| Set(Some(v))),
            is_enabled: req.is_enabled.map_or(NotSet, |v| Set(v)),
            is_public: req.is_public.map_or(NotSet, |v| Set(v)),

            member_limit_min: req.member_limit_min.map_or(NotSet, |v| Set(v)),
            member_limit_max: req.member_limit_max.map_or(NotSet, |v| Set(v)),
            parallel_container_limit: req.parallel_container_limit.map_or(NotSet, |v| Set(v)),

            is_need_write_up: req.is_need_write_up.map_or(NotSet, |v| Set(v)),
            started_at: req.started_at.map_or(NotSet, |v| Set(v)),
            ended_at: req.ended_at.map_or(NotSet, |v| Set(v)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub status: Option<crate::model::submission::Status>,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::challenge::Category;

use super::Metadata;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category: Option<Category>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub code: u16,
    pub data: Vec<crate::model::challenge::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusRequest {
    pub cids: Vec<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<crate::model::submission::Model>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub code: u16,
    pub data: HashMap<i64, StatusResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub description: String,
    pub category: Category,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub difficulty: Option<i64>,
    pub image_name: Option<String>,
    pub cpu_limit: Option<i64>,
    pub memory_limit: Option<i64>,
    pub duration: Option<i64>,
    pub ports: Option<Vec<i32>>,
    pub envs: Option<Vec<crate::model::challenge::Env>>,
    pub flags: Option<Vec<crate::model::challenge::Flag>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub code: u16,
    pub data: crate::model::challenge::Model,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub difficulty: Option<i64>,
    pub image_name: Option<String>,
    pub cpu_limit: Option<i64>,
    pub memory_limit: Option<i64>,
    pub duration: Option<i64>,
    pub ports: Option<Vec<i32>>,
    pub envs: Option<Vec<crate::model::challenge::Env>>,
    pub flags: Option<Vec<crate::model::challenge::Flag>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub code: u16,
    pub data: crate::model::challenge::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAttachmentMetadataResponse {
    pub code: u16,
    pub data: Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveAttachmentResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAttachmentResponse {
    pub code: u16,
}

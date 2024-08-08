use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub code: u16,
    pub data: Vec<crate::model::category::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateRequest {
    pub name: String,
    pub color: String,
    pub icon: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub code: u16,
    pub data: crate::model::category::Model,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub code: u16,
    pub data: crate::model::category::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub code: u16,
}

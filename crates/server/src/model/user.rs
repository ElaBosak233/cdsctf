use cds_db::{entity::user::Group, sea_orm, sea_orm::FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub group: Group,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct UserMini {
    pub id: i64,
    pub name: String,
    pub username: String,
}

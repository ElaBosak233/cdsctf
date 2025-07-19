use cds_db::{entity::challenge::Env, sea_orm, sea_orm::FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Vec<String>,
    pub is_dynamic: bool,
    pub has_attachment: bool,
    pub is_public: bool,
    pub env: Option<Env>,
    pub checker: Option<String>,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Challenge {
    pub fn desensitize(&self) -> Self {
        Self {
            env: None,
            checker: None,
            ..self.to_owned()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct ChallengeMini {
    pub id: uuid::Uuid,
    pub title: String,
    pub category: i32,
    pub tags: Vec<String>,
}

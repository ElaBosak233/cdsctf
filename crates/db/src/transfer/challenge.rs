use serde::{Deserialize, Serialize};

use crate::{entity, entity::challenge::Env};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<entity::challenge::Model> for Challenge {
    fn from(entity: entity::challenge::Model) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            description: entity.description,
            category: entity.category,
            tags: entity.tags,
            is_dynamic: entity.is_dynamic,
            has_attachment: entity.has_attachment,
            is_public: entity.is_public,
            env: entity.env,
            checker: entity.checker,
            deleted_at: entity.deleted_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
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

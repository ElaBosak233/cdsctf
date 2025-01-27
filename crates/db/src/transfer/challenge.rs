use std::str::FromStr;

use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

use crate::{
    entity,
    entity::challenge::{Env, Flag},
    get_db,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category: i32,
    pub tags: Vec<String>,
    pub is_dynamic: bool,
    pub has_attachment: bool,
    pub is_public: bool,
    pub env: Option<Env>,
    pub flags: Vec<Flag>,
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
            flags: entity.flags,
            deleted_at: entity.deleted_at,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<Challenge> for entity::challenge::Model {
    fn from(challenge: Challenge) -> Self {
        Self {
            id: challenge.id,
            title: challenge.title,
            description: challenge.description,
            category: challenge.category,
            tags: challenge.tags,
            is_dynamic: challenge.is_dynamic,
            has_attachment: challenge.has_attachment,
            is_public: challenge.is_public,
            env: challenge.env,
            flags: challenge.flags,
            deleted_at: challenge.deleted_at,
            created_at: challenge.created_at,
            updated_at: challenge.updated_at,
        }
    }
}

impl Challenge {
    pub fn desensitize(&mut self) {
        self.env = None;
        self.flags.clear();
    }
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<Challenge>, DbErr> {
    let models = entity::challenge::Entity::find()
        .filter(entity::challenge::Column::Id.is_in(ids))
        .all(get_db())
        .await?;
    let challenges = models.into_iter().map(Challenge::from).collect();
    Ok(challenges)
}

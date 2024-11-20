use sea_orm::{ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::db::{
    entity,
    entity::challenge::{Env, Flag},
    get_db,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Challenge {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub category: i32,
    pub tags: Vec<String>,
    pub is_dynamic: bool,
    pub has_attachment: bool,
    pub is_practicable: bool,
    pub image_name: Option<String>,
    pub cpu_limit: i64,
    pub memory_limit: i64,
    pub duration: i64,
    pub ports: Vec<i32>,
    pub envs: Vec<Env>,
    pub flags: Vec<Flag>,
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
            is_practicable: entity.is_practicable,
            image_name: entity.image_name,
            cpu_limit: entity.cpu_limit,
            memory_limit: entity.memory_limit,
            duration: entity.duration,
            ports: entity.ports,
            envs: entity.envs,
            flags: entity.flags,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl Challenge {
    pub fn desensitize(&mut self) {
        self.envs.clear();
        self.ports.clear();
        self.flags.clear();
    }
}

pub async fn find(
    id: Option<i64>, title: Option<String>, category: Option<i32>, is_practicable: Option<bool>,
    is_dynamic: Option<bool>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<Challenge>, u64), DbErr> {
    let mut sql = entity::challenge::Entity::find();

    if let Some(id) = id {
        sql = sql.filter(entity::challenge::Column::Id.eq(id));
    }

    if let Some(title) = title {
        sql = sql.filter(entity::challenge::Column::Title.contains(title));
    }

    if let Some(category) = category {
        sql = sql.filter(entity::challenge::Column::Category.eq(category));
    }

    if let Some(is_practicable) = is_practicable {
        sql = sql.filter(entity::challenge::Column::IsPracticable.eq(is_practicable));
    }

    if let Some(is_dynamic) = is_dynamic {
        sql = sql.filter(entity::challenge::Column::IsDynamic.eq(is_dynamic));
    }

    let total = sql.clone().count(get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let models = sql.all(get_db()).await?;
    let challenges = models.into_iter().map(Challenge::from).collect();

    Ok((challenges, total))
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<Challenge>, DbErr> {
    let models = entity::challenge::Entity::find()
        .filter(entity::challenge::Column::Id.is_in(ids))
        .all(get_db())
        .await?;
    let challenges = models.into_iter().map(Challenge::from).collect();
    Ok(challenges)
}

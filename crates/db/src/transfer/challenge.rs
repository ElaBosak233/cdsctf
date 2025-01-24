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
    pub is_deleted: bool,
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
            is_deleted: entity.is_deleted,
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
            is_deleted: challenge.is_deleted,
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

pub async fn find(
    id: Option<uuid::Uuid>, title: Option<String>, category: Option<i32>, is_public: Option<bool>,
    is_dynamic: Option<bool>, is_deleted: Option<bool>, sorts: Option<String>, page: Option<u64>,
    size: Option<u64>,
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

    if let Some(is_public) = is_public {
        sql = sql.filter(entity::challenge::Column::IsPublic.eq(is_public));
    }

    if let Some(is_dynamic) = is_dynamic {
        sql = sql.filter(entity::challenge::Column::IsDynamic.eq(is_dynamic));
    }

    match is_deleted {
        Some(true) => sql = sql.filter(entity::challenge::Column::IsDeleted.eq(true)),
        _ => sql = sql.filter(entity::challenge::Column::IsDeleted.eq(false)),
    }

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col =
                match crate::entity::challenge::Column::from_str(sort.replace("-", "").as_str()) {
                    Ok(col) => col,
                    Err(_) => continue,
                };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
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

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityName, EntityTrait, FromQueryResult, Iden as _, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, prelude::Expr,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::challenge::{ActiveModel, Container, Env, EnvVar, Model, Port};
pub(crate) use crate::entity::challenge::{Column, Entity};
use crate::{get_db, traits::DbError};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Challenge {
    pub id: i64,
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

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct ChallengeMini {
    pub id: i64,
    pub title: String,
    pub category: i32,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct FindChallengeOptions {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn find<T>(
    FindChallengeOptions {
        id,
        title,
        category,
        tag,
        is_public,
        is_dynamic,
        page,
        size,
        sorts,
    }: FindChallengeOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(title) = title {
        sql = sql.filter(Column::Title.contains(title));
    }

    if let Some(category) = category {
        sql = sql.filter(Column::Category.eq(category));
    }

    if let Some(tag) = tag {
        sql = sql.filter(Expr::cust_with_expr(
            format!(
                "\"{}\".\"{}\" @> $1::text[]",
                Entity.table_name(),
                Column::Tags.to_string()
            )
            .as_str(),
            vec![tag],
        ))
    }

    if let Some(is_public) = is_public {
        sql = sql.filter(Column::IsPublic.eq(is_public));
    }

    if let Some(is_dynamic) = is_dynamic {
        sql = sql.filter(Column::IsDynamic.eq(is_dynamic));
    }

    sql = sql.filter(Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match Column::from_str(sort.replace("-", "").as_str()) {
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

    let challenges = sql.into_model::<T>().all(get_db()).await?;

    Ok((challenges, total))
}

pub async fn find_by_id<T>(challenge_id: i64) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(challenge_id)
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn count() -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::DeletedAt.is_null())
        .count(get_db())
        .await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let challenge = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(challenge.id).await?.unwrap())
}

pub async fn update<T>(model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let challenge = model.update(get_db()).await?;

    Ok(find_by_id::<T>(challenge.id).await?.unwrap())
}

pub async fn delete(challenge_id: i64) -> Result<(), DbError> {
    let challenge = find_by_id::<Model>(challenge_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{challenge_id}")))?;

    let _ = ActiveModel {
        id: Set(challenge.id),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc().unix_timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(())
}

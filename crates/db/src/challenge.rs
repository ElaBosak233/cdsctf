use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityName, EntityTrait, FromQueryResult,
    Iden as _, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, prelude::Expr,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::challenge::{ActiveModel, Container, Env, EnvVar, Model, Port};
pub(crate) use crate::entity::challenge::{Column, Entity};
use crate::traits::DbError;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Challenge {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Vec<String>,
    pub dynamic: bool,
    pub has_attachment: bool,
    pub public: bool,
    pub has_writeup: bool,
    pub env: Option<Env>,
    pub checker: Option<String>,
    pub writeup: Option<String>,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Challenge {
    pub fn desensitize(&self) -> Self {
        Self {
            env: None,
            checker: None,
            writeup: if self.has_writeup && self.public {
                self.writeup.clone()
            } else {
                None
            },
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
    pub public: Option<bool>,
    pub dynamic: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindChallengeOptions {
        id,
        title,
        category,
        tag,
        public,
        dynamic,
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

    if let Some(public) = public {
        sql = sql.filter(Column::Public.eq(public));
    }

    if let Some(dynamic) = dynamic {
        sql = sql.filter(Column::Dynamic.eq(dynamic));
    }

    sql = sql.filter(Column::DeletedAt.is_null());

    let total = sql.clone().count(conn).await?;

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

    let challenges = sql.into_model::<T>().all(conn).await?;

    Ok((challenges, total))
}

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    challenge_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(challenge_id)
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(conn)
        .await?)
}

pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::DeletedAt.is_null())
        .count(conn)
        .await?)
}

pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let challenge = model.insert(conn).await?;

    Ok(find_by_id::<T>(conn, challenge.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{}", challenge.id)))?)
}

pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let challenge = model.update(conn).await?;

    Ok(find_by_id::<T>(conn, challenge.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{}", challenge.id)))?)
}

pub async fn delete(conn: &impl ConnectionTrait, challenge_id: i64) -> Result<(), DbError> {
    let challenge = find_by_id::<Model>(conn, challenge_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{challenge_id}")))?;

    let _ = ActiveModel {
        id: Set(challenge.id),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc().unix_timestamp())),
        ..Default::default()
    }
    .update(conn)
    .await?;

    Ok(())
}

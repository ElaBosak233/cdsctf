//! Database access for `note` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::note::ActiveModel;
pub(crate) use crate::entity::note::{Column, Entity};
use crate::{sea_orm, sea_orm::FromQueryResult, traits::DbError};

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub public: bool,
    pub user_id: i64,
    pub user_name: String,
    pub user_has_avatar: bool,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default)]
pub struct FindNotesOptions {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub public: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindNotesOptions {
        id,
        user_id,
        challenge_id,
        public,
        page,
        size,
        sorts,
    }: FindNotesOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::base_find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(Column::UserId.eq(user_id));
    }

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(public) = public {
        sql = sql.filter(Column::Public.eq(public));
    }

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

    let total = sql.clone().count(conn).await?;

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let notes = sql.into_model::<T>().all(conn).await?;

    Ok((notes, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    note_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::Id.eq(note_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Looks up by user id and challenge id.

pub async fn find_by_user_id_and_challenge_id<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
    challenge_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let note = model.insert(conn).await?;

    Ok(find_by_id::<T>(conn, note.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("note_{}", note.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let note = model.update(conn).await?;

    Ok(find_by_id::<T>(conn, note.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("note_{}", note.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, note_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(note_id).exec(conn).await?;

    Ok(())
}

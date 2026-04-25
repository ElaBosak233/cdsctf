//! Database access for `user` — SeaORM queries, updates, and DTOs.

use std::{fmt::Debug, str::FromStr};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityName, EntityTrait,
    FromQueryResult, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, Unchanged,
    prelude::Expr,
    sea_query::{Func, Query},
};
use serde::{Deserialize, Serialize};
use tracing::info;

pub use super::team_user::find_users as find_by_team_id;
pub use crate::entity::user::{ActiveModel, Group, Model};
pub(crate) use crate::entity::user::{Column, Entity};
use crate::{Email, traits::DbError};

#[allow(dead_code)]
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub verified: Option<bool>,
    pub group: Group,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    #[schema(ignore)]
    pub hashed_password: String,
    pub has_avatar: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[allow(dead_code)]
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserMini {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub has_avatar: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FindUserOptions {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindUserOptions {
        id,
        name,
        group,
        page,
        size,
        sorts,
    }: FindUserOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::base_find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(Column::Username.like(&pattern))
            .add(Column::Name.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(group) = group {
        sql = sql.filter(Column::Group.eq(group));
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

    let users = sql.into_model::<T>().all(conn).await?;

    Ok((users, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::Id.eq(user_id))
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Looks up by account.

pub async fn find_by_account<T>(
    conn: &impl ConnectionTrait,
    account: String,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult + Debug, {
    Ok(Entity::base_find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(Column::Username))).eq(account.to_lowercase()),
                )
                .add(Expr::exists(
                    Query::select()
                        .expr(Expr::val(1))
                        .from(crate::entity::email::Entity.table_name())
                        .and_where(
                            Expr::col((
                                crate::entity::email::Entity.table_name(),
                                crate::entity::email::Column::UserId,
                            ))
                            .eq(Expr::col((Entity.table_name(), Column::Id))),
                        )
                        .and_where(
                            Expr::expr(Func::lower(Expr::col((
                                crate::entity::email::Entity.table_name(),
                                crate::entity::email::Column::Email,
                            ))))
                            .eq(account.to_lowercase()),
                        )
                        .and_where(
                            Expr::col((
                                crate::entity::email::Entity.table_name(),
                                crate::entity::email::Column::Verified,
                            ))
                            .eq(true),
                        )
                        .to_owned(),
                )),
        )
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Looks up by email.

pub async fn find_by_email<T>(
    conn: &impl ConnectionTrait,
    email: String,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Expr::exists(
            Query::select()
                .expr(Expr::val(1))
                .from(crate::entity::email::Entity.table_name())
                .and_where(
                    Expr::col((
                        crate::entity::email::Entity.table_name(),
                        crate::entity::email::Column::UserId,
                    ))
                    .eq(Expr::col((Entity.table_name(), Column::Id))),
                )
                .and_where(
                    Expr::expr(Func::lower(Expr::col((
                        crate::entity::email::Entity.table_name(),
                        crate::entity::email::Column::Email,
                    ))))
                    .eq(email.to_lowercase()),
                )
                .to_owned(),
        ))
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::DeletedAt.is_null())
        .count(conn)
        .await?)
}

/// Returns whether is username unique.

pub async fn is_username_unique(
    conn: &impl ConnectionTrait,
    user_id: i64,
    username: &str,
) -> Result<bool, DbError> {
    let user = Entity::base_find()
        .filter(Expr::expr(Func::lower(Expr::col(Column::Username))).eq(username.to_lowercase()))
        .one(conn)
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

/// Returns whether is email unique.

pub async fn is_email_unique(conn: &impl ConnectionTrait, email: &str) -> Result<bool, DbError> {
    Ok(crate::email::find_by_email::<Email>(conn, email.to_owned())
        .await?
        .is_none())
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let user = model.insert(conn).await?;
    info!(
        user_id = user.id,
        username = %user.username,
        group = ?user.group,
        "user created"
    );

    Ok(find_by_id::<T>(conn, user.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{}", user.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let user = model.update(conn).await?;
    info!(
        user_id = user.id,
        username = %user.username,
        group = ?user.group,
        "user updated"
    );

    Ok(find_by_id::<T>(conn, user.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{}", user.id)))?)
}

/// Updates password.

pub async fn update_password(
    conn: &impl ConnectionTrait,
    user_id: i64,
    hashed_password: String,
) -> Result<(), DbError> {
    let _ = update::<Model>(
        conn,
        ActiveModel {
            id: Unchanged(user_id),
            hashed_password: Set(hashed_password),
            ..Default::default()
        },
    )
    .await?;
    info!(user_id, "user password updated");

    Ok(())
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, user_id: i64) -> Result<(), DbError> {
    let user = find_by_id::<Model>(conn, user_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{user_id}")))?;

    let _ = ActiveModel {
        id: Unchanged(user.id),
        username: Set(format!("[DELETED]_{}_{}", user.id, user.username)),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc().unix_timestamp())),
        ..Default::default()
    }
    .update(conn)
    .await?;

    let _ = super::email::delete_by_user_id(conn, user_id).await?;
    info!(
        user_id,
        username = %user.username,
        "user deleted"
    );

    Ok(())
}

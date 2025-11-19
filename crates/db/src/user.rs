use std::{fmt::Debug, str::FromStr};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityName, EntityTrait, FromQueryResult,
    Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, Unchanged,
    prelude::Expr,
    sea_query::{Func, Query},
};
use serde::{Deserialize, Serialize};

pub use super::team_user::find_users as find_by_team_id;
pub use crate::entity::user::{ActiveModel, Group, Model};
pub(crate) use crate::entity::user::{Column, Entity};
use crate::{Email, get_db, traits::DbError};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub is_verified: Option<bool>,
    pub group: Group,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub has_avatar: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
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

pub async fn find<T>(
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

    let users = sql.into_model::<T>().all(get_db()).await?;

    Ok((users, total))
}

pub async fn find_by_id<T>(user_id: i64) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::Id.eq(user_id))
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn find_by_account<T>(account: String) -> Result<Option<T>, DbError>
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
                                crate::entity::email::Column::IsVerified,
                            ))
                            .eq(true),
                        )
                        .to_owned(),
                )),
        )
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn find_by_email<T>(email: String) -> Result<Option<T>, DbError>
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
        .one(get_db())
        .await?)
}

pub async fn count() -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::DeletedAt.is_null())
        .count(get_db())
        .await?)
}

pub async fn is_username_unique(user_id: i64, username: &str) -> Result<bool, DbError> {
    let user = Entity::base_find()
        .filter(Expr::expr(Func::lower(Expr::col(Column::Username))).eq(username.to_lowercase()))
        .one(get_db())
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

pub async fn is_email_unique(email: &str) -> Result<bool, DbError> {
    Ok(crate::email::find_by_email::<Email>(email.to_owned())
        .await?
        .is_none())
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let user = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(user.id).await?.unwrap())
}

pub async fn update<T>(model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let user = model.update(get_db()).await?;

    Ok(find_by_id::<T>(user.id).await?.unwrap())
}

pub async fn update_password(user_id: i64, hashed_password: String) -> Result<(), DbError> {
    let _ = update::<Model>(ActiveModel {
        id: Unchanged(user_id),
        hashed_password: Set(hashed_password),
        ..Default::default()
    })
    .await?;

    Ok(())
}

pub async fn delete(user_id: i64) -> Result<(), DbError> {
    let user = find_by_id::<Model>(user_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{user_id}")))?;

    let _ = ActiveModel {
        id: Unchanged(user.id),
        username: Set(format!("[DELETED]_{}_{}", user.id, user.username)),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc().unix_timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    let _ = super::email::delete_by_user_id(user_id).await?;

    Ok(())
}

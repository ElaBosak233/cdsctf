use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, FromQueryResult, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, Unchanged, prelude::Expr,
    sea_query::Func,
};
use serde::{Deserialize, Serialize};

use crate::{entity::user::Group, get_db};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub group: Group,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct UserMini {
    pub id: i64,
    pub name: String,
    pub username: String,
}

pub struct FindUserOptions {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn find<T>(
    FindUserOptions {
        id,
        name,
        email,
        group,
        page,
        size,
        sorts,
    }: FindUserOptions,
) -> Result<(Vec<T>, u64), DbErr>
where
    T: FromQueryResult, {
    let mut sql = crate::entity::user::Entity::find();

    if let Some(id) = id {
        sql = sql.filter(crate::entity::user::Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(crate::entity::user::Column::Username.like(&pattern))
            .add(crate::entity::user::Column::Name.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(group) = group {
        sql = sql.filter(crate::entity::user::Column::Group.eq(group));
    }

    if let Some(email) = email {
        sql = sql.filter(crate::entity::user::Column::Email.eq(email));
    }

    sql = sql.filter(crate::entity::user::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match crate::entity::user::Column::from_str(sort.replace("-", "").as_str()) {
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

pub async fn find_by_id<T>(user_id: i64) -> Result<Option<T>, DbErr>
where
    T: FromQueryResult, {
    Ok(crate::entity::user::Entity::find_by_id(user_id)
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn is_username_unique(user_id: i64, username: &str) -> Result<bool, DbErr> {
    let user = crate::entity::user::Entity::find()
        .filter(
            Expr::expr(Func::lower(Expr::col(
                crate::entity::user::Column::Username,
            )))
            .eq(username.to_lowercase()),
        )
        .one(get_db())
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

pub async fn is_email_unique(user_id: i64, email: &str) -> Result<bool, DbErr> {
    let user = crate::entity::user::Entity::find()
        .filter(
            Expr::expr(Func::lower(Expr::col(crate::entity::user::Column::Email)))
                .eq(email.to_lowercase()),
        )
        .one(get_db())
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

pub async fn create<T>(model: crate::entity::user::ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult, {
    let user = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(user.id).await?.unwrap())
}

pub async fn update<T>(model: crate::entity::user::ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult, {
    let user = model.update(get_db()).await?;

    Ok(find_by_id::<T>(user.id).await?.unwrap())
}

pub async fn update_password(user_id: i64, hashed_password: String) -> Result<(), DbErr> {
    let _ = update::<crate::entity::user::Model>(crate::entity::user::ActiveModel {
        id: Unchanged(user_id),
        hashed_password: Set(hashed_password),
        ..Default::default()
    })
    .await?;

    Ok(())
}

pub async fn delete(user_id: i64) -> Result<(), DbErr> {
    let user = find_by_id::<crate::entity::user::Model>(user_id)
        .await?
        .unwrap();

    let _ = update::<crate::entity::user::Model>(crate::entity::user::ActiveModel {
        id: Unchanged(user.id),
        username: Set(format!("[DELETED]_{}", user.username)),
        email: Set(format!("deleted_{}@del.cdsctf", user.email)),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc().unix_timestamp())),
        ..Default::default()
    })
    .await?;

    Ok(())
}

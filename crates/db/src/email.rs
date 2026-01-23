use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::email::{ActiveModel, Model};
pub(crate) use crate::entity::email::{Column, Entity};
use crate::traits::DbError;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Email {
    pub email: String,
    pub is_verified: bool,
    pub user_id: i64,
}

pub async fn find_by_user_id<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    let emails = Entity::find()
        .filter(Column::UserId.eq(user_id))
        .into_model::<T>()
        .all(conn)
        .await?;

    Ok(emails)
}

pub async fn find_by_email<T>(
    conn: &impl ConnectionTrait,
    email: String,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    let email = Entity::find_by_id(email)
        .into_model::<T>()
        .one(conn)
        .await?;

    Ok(email)
}

pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let email = model.insert(conn).await?;

    Ok(find_by_email::<T>(conn, email.email.clone())
        .await?
        .ok_or_else(|| DbError::NotFound(format!("email_{}", email.email)))?)
}

pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let email = model.update(conn).await?;

    Ok(find_by_email::<T>(conn, email.email.clone())
        .await?
        .ok_or_else(|| DbError::NotFound(format!("email_{}", email.email)))?)
}

pub async fn delete(
    conn: &impl ConnectionTrait,
    user_id: i64,
    email: String,
) -> Result<(), DbError> {
    let email = Entity::find()
        .filter(Column::Email.eq(email.clone()))
        .filter(Column::UserId.eq(user_id))
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("email_{user_id}_{email}")))?;

    if Entity::find()
        .filter(Column::UserId.eq(email.user_id))
        .count(conn)
        .await?
        <= 1
    {
        return Err(DbError::BadRequest("user_has_no_other_emails".to_string()));
    }

    let _ = Entity::delete_many()
        .filter(Column::Email.eq(email.email))
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn delete_by_user_id(conn: &impl ConnectionTrait, user_id: i64) -> Result<(), DbError> {
    let _ = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .exec(conn)
        .await?;

    Ok(())
}

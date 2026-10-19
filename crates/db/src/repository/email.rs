//! Database access for `email` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter,
};

pub(crate) use crate::entity::email::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::email::EmailView,
    entity::email::{ActiveModel, Model},
};

/// Looks up by user id.

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

/// Looks up by email.

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

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let email = model.insert(conn).await?;
    tracing::info!(
        resource = "email",
        action = "create",
        user_id = email.user_id,
        email = ?email.email,
        verified = ?email.verified,
        "database step completed",
    );

    Ok(find_by_email::<T>(conn, email.email.clone())
        .await?
        .ok_or_else(|| DbError::NotFound(format!("email_{}", email.email)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let changed_fields = [
        ("email", model.email.is_set()),
        ("verified", model.verified.is_set()),
        ("user_id", model.user_id.is_set()),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect::<Vec<_>>();
    let email = model.update(conn).await?;
    tracing::info!(
        resource = "email",
        action = "update",
        changed_fields = ?changed_fields,
        user_id = email.user_id,
        email = ?email.email,
        verified = ?email.verified,
        "database step completed",
    );

    Ok(find_by_email::<T>(conn, email.email.clone())
        .await?
        .ok_or_else(|| DbError::NotFound(format!("email_{}", email.email)))?)
}

/// Deletes rows matching the provided identifier or filter.
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
        .filter(Column::Email.eq(email.email.clone()))
        .exec(conn)
        .await?;
    tracing::info!(
        resource = "email",
        action = "delete",
        user_id,
        email = ?email.email,
        "database step completed",
    );

    Ok(())
}

/// Deletes by user id.

pub async fn delete_by_user_id(conn: &impl ConnectionTrait, user_id: i64) -> Result<(), DbError> {
    let _ = Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .exec(conn)
        .await?;
    tracing::info!(
        resource = "email",
        action = "delete",
        user_id = user_id,
        "resource step completed"
    );

    Ok(())
}

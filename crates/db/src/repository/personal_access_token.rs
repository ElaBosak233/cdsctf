//! Database access for user-bound personal access tokens.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter,
    QueryOrder, Set, Unchanged,
};
use tracing::info;

pub(crate) use crate::entity::personal_access_token::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::personal_access_token::PersonalAccessTokenView,
    entity::personal_access_token::{ActiveModel, Model},
};

pub async fn find_by_user_id<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::UserId.eq(user_id))
        .order_by_desc(Column::CreatedAt)
        .into_model::<T>()
        .all(conn)
        .await?)
}

pub async fn find_by_token_id(
    conn: &impl ConnectionTrait,
    token_id: &str,
) -> Result<Option<Model>, DbError> {
    Ok(Entity::find()
        .filter(Column::TokenId.eq(token_id))
        .one(conn)
        .await?)
}

pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let token = model.insert(conn).await?;
    info!(user_id = token.user_id, token_id = %token.token_id, "personal access token created");
    Ok(Entity::find_by_id(token.id)
        .into_model::<T>()
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("personal_access_token_{}", token.id)))?)
}

pub async fn revoke(conn: &impl ConnectionTrait, user_id: i64, id: i64) -> Result<(), DbError> {
    let token = Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("personal_access_token_{id}")))?;

    ActiveModel {
        id: Unchanged(token.id),
        revoked_at: Set(Some(time::OffsetDateTime::now_utc())),
        ..Default::default()
    }
    .update(conn)
    .await?;
    info!(user_id, token_id = %token.token_id, "personal access token revoked");
    Ok(())
}

pub async fn touch_last_used(conn: &impl ConnectionTrait, id: i64) -> Result<(), DbError> {
    ActiveModel {
        id: Unchanged(id),
        last_used_at: Set(Some(time::OffsetDateTime::now_utc())),
        ..Default::default()
    }
    .update(conn)
    .await?;
    Ok(())
}

pub async fn delete_by_user_id(conn: &impl ConnectionTrait, user_id: i64) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .exec(conn)
        .await?;
    Ok(())
}

//! Database access for `game_notice` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::game_notice::{ActiveModel, Model};
pub(crate) use crate::entity::game_notice::{Column, Entity};
use crate::traits::DbError;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameNotice {
    pub id: i64,
    pub game_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    notice_id: i64,
    game_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(notice_id)
        .filter(Column::GameId.eq(game_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Looks up by game id.

pub async fn find_by_game_id<T>(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::GameId.eq(game_id))
        .into_model::<T>()
        .all(conn)
        .await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game_notice = model.insert(conn).await?;

    Ok(find_by_id::<T>(conn, game_notice.id, game_notice.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_notice_{}", game_notice.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(
    conn: &impl ConnectionTrait,
    notice_id: i64,
    game_id: i64,
) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::Id.eq(notice_id))
        .filter(Column::GameId.eq(game_id))
        .exec(conn)
        .await?;

    Ok(())
}

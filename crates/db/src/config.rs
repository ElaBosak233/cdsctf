//! Database access for `config` — SeaORM queries, updates, and DTOs.

use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, Set, sea_query::OnConflict};

pub use crate::entity::config::{ActiveModel, Config, Model, auth, captcha, email, meta};
pub(crate) use crate::entity::config::{Column, Entity};
use crate::traits::DbError;

/// Loads the singleton `configs` row and returns the embedded JSON
/// configuration DTO.
pub async fn get(conn: &impl ConnectionTrait) -> Result<Config, DbError> {
    Ok(Entity::find()
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound("config".to_string()))?
        .data)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Upserts the singleton `configs` row with a new JSON payload (insert or
/// on-conflict update).
pub async fn save(conn: &impl ConnectionTrait, config: Config) -> Result<Config, DbError> {
    let model = ActiveModel {
        id: Set(true),
        data: Set(config),
    };

    Entity::insert(model.clone())
        .on_conflict(
            OnConflict::column(Column::Id)
                .update_column(Column::Data)
                .to_owned(),
        )
        .exec(conn)
        .await?;

    Ok(get(conn).await?)
}

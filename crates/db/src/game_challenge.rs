//! Database access for `game_challenge` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};

pub(crate) use crate::entity::game_challenge::Entity;
pub use crate::entity::game_challenge::{ActiveModel, Column, Model, Relation};
use crate::traits::DbError;

#[allow(dead_code)]
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameChallenge {
    pub game_id: i64,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub difficulty: i64,
    pub bonus_ratios: Vec<i64>,
    pub max_pts: i64,
    pub min_pts: i64,
    pub pts: i64,
    pub enabled: bool,
    pub frozen_at: Option<i64>,
}

#[allow(dead_code)]
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameChallengeMini {
    pub game_id: i64,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub pts: i64,
    pub frozen_at: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct FindGameChallengeOptions {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub enabled: Option<bool>,
    pub category: Option<i32>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindGameChallengeOptions {
        game_id,
        challenge_id,
        enabled,
        category,
    }: FindGameChallengeOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    // Using inner join to access fields in related tables.
    let mut sql = Entity::base_find();

    sql = sql.filter(Column::GameId.eq(game_id));

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(enabled) = enabled {
        sql = sql.filter(Column::Enabled.eq(enabled));
    }

    if let Some(category) = category {
        sql = sql.filter(super::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(conn).await?;

    let game_challenges = sql.into_model::<T>().all(conn).await?;

    Ok((game_challenges, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    game_id: i64,
    challenge_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game_challenge = model.insert(conn).await?;

    Ok(
        find_by_id::<T>(conn, game_challenge.game_id, game_challenge.challenge_id)
            .await?
            .ok_or_else(|| {
                DbError::NotFound(format!(
                    "game_challenge_{}_{}",
                    game_challenge.game_id, game_challenge.challenge_id
                ))
            })?,
    )
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game_challenge = model.update(conn).await?;

    Ok(
        find_by_id::<T>(conn, game_challenge.game_id, game_challenge.challenge_id)
            .await?
            .ok_or_else(|| {
                DbError::NotFound(format!(
                    "game_challenge_{}_{}",
                    game_challenge.game_id, game_challenge.challenge_id
                ))
            })?,
    )
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(
    conn: &impl ConnectionTrait,
    game_id: i64,
    challenge_id: i64,
) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .exec(conn)
        .await?;

    Ok(())
}

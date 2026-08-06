//! Database access for `game_challenge` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};
use tracing::info;

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

impl TryFrom<crate::entity::game_challenge::ModelEx> for GameChallenge {
    type Error = DbError;

    fn try_from(
        game_challenge: crate::entity::game_challenge::ModelEx,
    ) -> Result<Self, Self::Error> {
        let challenge = game_challenge.challenge.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "game challenge {}/{} was loaded without its challenge relation",
                game_challenge.game_id,
                game_challenge.challenge_id
            ))
        })?;

        Ok(Self {
            game_id: game_challenge.game_id,
            challenge_id: game_challenge.challenge_id,
            challenge_title: challenge.title.clone(),
            challenge_category: challenge.category,
            difficulty: game_challenge.difficulty,
            bonus_ratios: game_challenge.bonus_ratios,
            max_pts: game_challenge.max_pts,
            min_pts: game_challenge.min_pts,
            pts: game_challenge.pts,
            enabled: game_challenge.enabled,
            frozen_at: game_challenge.frozen_at,
        })
    }
}

impl From<GameChallenge> for GameChallengeMini {
    fn from(game_challenge: GameChallenge) -> Self {
        Self {
            game_id: game_challenge.game_id,
            challenge_id: game_challenge.challenge_id,
            challenge_title: game_challenge.challenge_title,
            challenge_category: game_challenge.challenge_category,
            pts: game_challenge.pts,
            frozen_at: game_challenge.frozen_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameChallengeMini;

    #[test]
    fn mini_serialization_excludes_admin_scoring_configuration() {
        let game_challenge = GameChallengeMini {
            game_id: 1,
            challenge_id: 2,
            challenge_title: "challenge".to_owned(),
            challenge_category: 3,
            pts: 100,
            frozen_at: Some(1_700_000_000),
        };

        assert_eq!(
            serde_json::to_value(game_challenge).unwrap(),
            serde_json::json!({
                "game_id": 1,
                "challenge_id": 2,
                "challenge_title": "challenge",
                "challenge_category": 3,
                "pts": 100,
                "frozen_at": 1_700_000_000_i64,
            })
        );
    }
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
    T: From<GameChallenge>, {
    let mut loader = Entity::load()
        .with(crate::entity::challenge::Entity)
        .filter(Column::GameId.eq(game_id));

    if let Some(challenge_id) = challenge_id {
        loader = loader.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(enabled) = enabled {
        loader = loader.filter(Column::Enabled.eq(enabled));
    }

    if let Some(category) = category {
        let challenge_ids = super::challenge::Entity::find()
            .filter(super::challenge::Column::Category.eq(category))
            .select_only()
            .column(super::challenge::Column::Id)
            .into_tuple::<i64>()
            .all(conn)
            .await?;
        loader = loader.filter(Column::ChallengeId.is_in(challenge_ids));
    }

    let models = loader.all(conn).await?;
    let total = models.len() as u64;
    let game_challenges = models
        .into_iter()
        .map(GameChallenge::try_from)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(T::from)
        .collect();

    Ok((game_challenges, total))
}

/// Looks up by id.

pub async fn find_by_id(
    conn: &impl ConnectionTrait,
    game_id: i64,
    challenge_id: i64,
) -> Result<Option<GameChallenge>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::challenge::Entity)
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .one(conn)
        .await?
        .map(GameChallenge::try_from)
        .transpose()?)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<GameChallenge, DbError> {
    let game_challenge = model.insert(conn).await?;
    info!(
        game_id = game_challenge.game_id,
        challenge_id = game_challenge.challenge_id,
        enabled = game_challenge.enabled,
        pts = game_challenge.pts,
        "game challenge created"
    );

    Ok(
        find_by_id(conn, game_challenge.game_id, game_challenge.challenge_id)
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
pub async fn update(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<GameChallenge, DbError> {
    let game_challenge = model.update(conn).await?;
    info!(
        game_id = game_challenge.game_id,
        challenge_id = game_challenge.challenge_id,
        enabled = game_challenge.enabled,
        pts = game_challenge.pts,
        "game challenge updated"
    );

    Ok(
        find_by_id(conn, game_challenge.game_id, game_challenge.challenge_id)
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
    info!(game_id, challenge_id, "game challenge deleted");

    Ok(())
}

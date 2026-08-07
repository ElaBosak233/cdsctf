//! Database access for `game_challenge` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
    sea_query::{Alias, Expr, ExprTrait, IntoIden, Query, TableRef, UpdateStatement, ValueTuple},
};
use tracing::info;

pub(crate) use crate::entity::game_challenge::Entity;
use crate::traits::DbError;
pub use crate::{
    dto::game_challenge::{GameChallengeSummary, GameChallengeView},
    entity::game_challenge::{ActiveModel, Column, Model, Relation},
};

impl TryFrom<crate::entity::game_challenge::ModelEx> for GameChallengeView {
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

impl From<GameChallengeView> for GameChallengeSummary {
    fn from(game_challenge: GameChallengeView) -> Self {
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

#[derive(Clone, Debug, Default)]
pub struct FindGameChallengeOptions {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub enabled: Option<bool>,
    pub category: Option<i32>,
}

/// Returns whether a challenge belongs to a game.
pub async fn exists(
    conn: &impl ConnectionTrait,
    game_id: i64,
    challenge_id: i64,
) -> Result<bool, DbError> {
    Ok(membership_query(game_id, challenge_id).count(conn).await? > 0)
}

fn membership_query(game_id: i64, challenge_id: i64) -> sea_orm::Select<Entity> {
    Entity::find()
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
}

/// Narrow game-challenge projection used by score recomputation.
#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
pub struct ScoreInput {
    pub challenge_id: i64,
    pub difficulty: i64,
    pub max_pts: i64,
    pub min_pts: i64,
    pub bonus_ratios: Vec<i64>,
    pub pts: i64,
}

/// Persisted score fields for one game challenge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreUpdate {
    pub challenge_id: i64,
    pub pts: i64,
}

/// Loads challenge scoring configuration without challenge relations.
pub async fn find_score_inputs(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<ScoreInput>, DbError> {
    Ok(Entity::find()
        .select_only()
        .columns([
            Column::ChallengeId,
            Column::Difficulty,
            Column::MaxPts,
            Column::MinPts,
            Column::BonusRatios,
            Column::Pts,
        ])
        .filter(Column::GameId.eq(game_id))
        .order_by_asc(Column::ChallengeId)
        .into_model::<ScoreInput>()
        .all(conn)
        .await?)
}

/// Applies changed challenge scores in bounded, set-based updates.
pub async fn update_scores(
    conn: &impl ConnectionTrait,
    game_id: i64,
    updates: &[ScoreUpdate],
) -> Result<u64, DbError> {
    let mut rows_affected = 0;
    for statement in score_update_statements(game_id, updates) {
        rows_affected += conn.execute(&statement).await?.rows_affected();
    }
    Ok(rows_affected)
}

fn score_update_statements(game_id: i64, updates: &[ScoreUpdate]) -> Vec<UpdateStatement> {
    updates
        .chunks(super::BULK_UPDATE_BATCH_SIZE)
        .map(|chunk| {
            let source = Alias::new("challenge_scores");
            let values = chunk
                .iter()
                .map(|update| {
                    ValueTuple::Many(vec![
                        game_id.into(),
                        update.challenge_id.into(),
                        update.pts.into(),
                    ])
                })
                .collect();
            let source_game_id = Expr::col((source.clone(), Alias::new("column1")));
            let source_challenge_id = Expr::col((source.clone(), Alias::new("column2")));
            let source_pts = Expr::col((source.clone(), Alias::new("column3")));

            Query::update()
                .table(Entity)
                .value(Column::Pts, source_pts.clone())
                .from(TableRef::ValuesList(values, source.into_iden()))
                .cond_where(Expr::col(Column::GameId).eq(source_game_id))
                .cond_where(Expr::col(Column::ChallengeId).eq(source_challenge_id))
                .cond_where(Expr::col(Column::Pts).ne(source_pts))
                .to_owned()
        })
        .collect()
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
    T: From<GameChallengeView>, {
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
        .map(GameChallengeView::try_from)
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
) -> Result<Option<GameChallengeView>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::challenge::Entity)
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .one(conn)
        .await?
        .map(GameChallengeView::try_from)
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
) -> Result<GameChallengeView, DbError> {
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
) -> Result<GameChallengeView, DbError> {
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

#[cfg(test)]
mod score_tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::*;

    #[test]
    fn score_updates_use_one_bound_values_statement() {
        let statements = score_update_statements(
            3,
            &[
                ScoreUpdate {
                    challenge_id: 10,
                    pts: 500,
                },
                ScoreUpdate {
                    challenge_id: 20,
                    pts: 400,
                },
            ],
        );

        let statement = DbBackend::Postgres.build(&statements[0]);
        assert_eq!(statements.len(), 1);
        assert_eq!(statement.values.unwrap().0.len(), 6);
        assert!(statement.sql.starts_with("UPDATE \"game_challenges\""));
        assert!(statement.sql.contains("FROM (VALUES"));
        assert!(statement.sql.contains("\"challenge_scores\".\"column3\""));
    }

    #[test]
    fn membership_query_is_scoped_to_game_and_challenge() {
        let statement = membership_query(3, 10).build(DbBackend::Postgres);

        assert!(statement.sql.contains("FROM \"game_challenges\""));
        assert!(statement.sql.contains("\"game_id\" = $1"));
        assert!(statement.sql.contains("\"challenge_id\" = $2"));
        assert_eq!(statement.values.unwrap().0.len(), 2);
    }
}

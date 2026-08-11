//! Database access for `submission` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityLoaderTrait, EntityTrait,
    FromQueryResult, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    sea_query::{
        Alias, Condition, Expr, ExprTrait, IntoIden, Query, TableRef, UpdateStatement, ValueTuple,
    },
};
use tracing::info;

pub(crate) use crate::entity::submission::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::{
        scoreboard::ScoreboardSubmission,
        submission::{SubmissionSummary, SubmissionView},
    },
    entity::submission::{ActiveModel, Status},
};

pub const PROCESSING_LEASE_SECONDS: i64 = 15;

impl TryFrom<crate::entity::submission::ModelEx> for SubmissionView {
    type Error = DbError;

    fn try_from(submission: crate::entity::submission::ModelEx) -> Result<Self, Self::Error> {
        let user = submission.user.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its user relation",
                submission.id
            ))
        })?;
        let challenge = submission.challenge.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its challenge relation",
                submission.id
            ))
        })?;

        let team = if submission.team_id.is_some() {
            Some(submission.team.as_ref().ok_or_else(|| {
                DbError::Other(anyhow::anyhow!(
                    "submission {} was loaded without its team relation",
                    submission.id
                ))
            })?)
        } else {
            None
        };
        let game = if submission.game_id.is_some() {
            Some(submission.game.as_ref().ok_or_else(|| {
                DbError::Other(anyhow::anyhow!(
                    "submission {} was loaded without its game relation",
                    submission.id
                ))
            })?)
        } else {
            None
        };

        Ok(Self {
            id: submission.id,
            content: submission.content,
            status: submission.status,
            user_id: submission.user_id,
            user_name: user.name.clone(),
            user_avatar_hash: user.avatar_hash.clone(),
            team_id: submission.team_id,
            team_name: team.map(|team| team.name.clone()),
            team_avatar_hash: team.and_then(|team| team.avatar_hash.clone()),
            game_id: submission.game_id,
            game_title: game.map(|game| game.title.clone()),
            challenge_id: submission.challenge_id,
            challenge_title: challenge.title.clone(),
            challenge_category: challenge.category,
            created_at: submission.created_at,
            processing_at: submission.processing_at,
            checked_at: submission.checked_at,
            pts: submission.pts,
            rank: submission.rank,
        })
    }
}

/// Projects a SeaORM 2 loaded submission graph to the public scoreboard
/// contract. The graph supplies the related user and challenge without a
/// second hand-written join DTO.
impl TryFrom<&crate::entity::submission::ModelEx> for ScoreboardSubmission {
    type Error = DbError;

    fn try_from(submission: &crate::entity::submission::ModelEx) -> Result<Self, Self::Error> {
        let user = submission.user.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its user relation",
                submission.id
            ))
        })?;
        let challenge = submission.challenge.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its challenge relation",
                submission.id
            ))
        })?;

        Ok(Self {
            id: submission.id,
            user_id: submission.user_id,
            user_name: user.name.clone(),
            user_avatar_hash: user.avatar_hash.clone(),
            challenge_id: submission.challenge_id,
            challenge_title: challenge.title.clone(),
            pts: submission.pts,
            created_at: submission.created_at,
        })
    }
}

impl From<&SubmissionView> for ScoreboardSubmission {
    fn from(submission: &SubmissionView) -> Self {
        Self {
            id: submission.id,
            user_id: submission.user_id,
            user_name: submission.user_name.clone(),
            user_avatar_hash: submission.user_avatar_hash.clone(),
            challenge_id: submission.challenge_id,
            challenge_title: submission.challenge_title.clone(),
            pts: submission.pts,
            created_at: submission.created_at,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FindSubmissionsOptions {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<Option<i64>>,
    pub game_id: Option<Option<i64>>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Narrow submission projection used by score recomputation.
#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
pub struct ScoreInput {
    pub id: i64,
    pub challenge_id: i64,
    pub team_id: Option<i64>,
    pub created_at: i64,
    pub pts: i64,
    pub rank: i64,
}

/// Persisted score fields for one submission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreUpdate {
    pub id: i64,
    pub pts: i64,
    pub rank: i64,
}

/// Loads correct submissions using only fields needed for score recomputation.
pub async fn find_score_inputs(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<ScoreInput>, DbError> {
    Ok(score_input_query(game_id)
        .into_model::<ScoreInput>()
        .all(conn)
        .await?)
}

fn score_input_query(game_id: i64) -> sea_orm::Select<Entity> {
    Entity::find()
        .select_only()
        .columns([
            Column::Id,
            Column::ChallengeId,
            Column::TeamId,
            Column::CreatedAt,
            Column::Pts,
            Column::Rank,
        ])
        .filter(Column::GameId.eq(game_id))
        .filter(Expr::col(Column::Status).eq(Expr::Constant("correct".into())))
        .order_by_asc(Column::ChallengeId)
        .order_by_asc(Column::CreatedAt)
        .order_by_asc(Column::Id)
}

/// Applies changed submission scores in bounded, set-based updates.
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
            let source = Alias::new("submission_scores");
            let values = chunk
                .iter()
                .map(|update| {
                    ValueTuple::Many(vec![
                        game_id.into(),
                        update.id.into(),
                        update.pts.into(),
                        update.rank.into(),
                    ])
                })
                .collect();
            let source_game_id = Expr::col((source.clone(), Alias::new("column1")));
            let source_id = Expr::col((source.clone(), Alias::new("column2")));
            let source_pts = Expr::col((source.clone(), Alias::new("column3")));
            let source_rank = Expr::col((source.clone(), Alias::new("column4")));

            Query::update()
                .table(Entity)
                .value(Column::Pts, source_pts.clone())
                .value(Column::Rank, source_rank.clone())
                .from(TableRef::ValuesList(values, source.into_iden()))
                .cond_where(Expr::col(Column::GameId).eq(source_game_id))
                .cond_where(Expr::col(Column::Id).eq(source_id))
                .cond_where(
                    Condition::any()
                        .add(Expr::col(Column::Pts).ne(source_pts))
                        .add(Expr::col(Column::Rank).ne(source_rank)),
                )
                .to_owned()
        })
        .collect()
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find(
    conn: &impl ConnectionTrait,
    FindSubmissionsOptions {
        id,
        user_id,
        team_id,
        game_id,
        challenge_id,
        status,
        page,
        size,
        sorts,
    }: FindSubmissionsOptions,
) -> Result<(Vec<SubmissionView>, u64), DbError> {
    let mut loader = Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity);

    if let Some(id) = id {
        loader = loader.filter(Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        loader = loader.filter(Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        match team_id {
            Some(team_id) => loader = loader.filter(Column::TeamId.eq(team_id)),
            None => loader = loader.filter(Column::TeamId.is_null()),
        }
    }

    if let Some(game_id) = game_id {
        match game_id {
            Some(game_id) => loader = loader.filter(Column::GameId.eq(game_id)),
            None => loader = loader.filter(Column::GameId.is_null()),
        }
    }

    if let Some(challenge_id) = challenge_id {
        loader = loader.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        loader = loader.filter(Column::Status.eq(status));
    }

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                loader = loader.order_by(col, Order::Desc);
            } else {
                loader = loader.order_by(col, Order::Asc);
            }
        }
    }

    let (models, total) = match (page, size) {
        (Some(_), Some(0)) => {
            let total = loader.clone().paginate(conn, 1).num_items().await?;
            (Vec::new(), total)
        }
        (Some(page), Some(size)) => {
            let paginator = loader.paginate(conn, size);
            let total = paginator.num_items().await?;
            let models = paginator.fetch_page(page.saturating_sub(1)).await?;
            (models, total)
        }
        _ => {
            let models = loader.all(conn).await?;
            let total = models.len() as u64;
            (models, total)
        }
    };

    let submissions = models
        .into_iter()
        .map(SubmissionView::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((submissions, total))
}

/// Looks up by id.

pub async fn find_by_id(
    conn: &impl ConnectionTrait,
    submission_id: i64,
) -> Result<Option<SubmissionView>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::Id.eq(submission_id))
        .one(conn)
        .await?
        .map(SubmissionView::try_from)
        .transpose()?)
}

/// Atomically claims a queued submission or takes over an expired processing
/// lease.
pub async fn claim_queued_or_stale_by_id(
    conn: &impl ConnectionTrait,
    submission_id: i64,
) -> Result<Option<SubmissionView>, DbError> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let cutoff = now.saturating_sub(PROCESSING_LEASE_SECONDS);
    let result = claimable_submission_query(submission_id, now, cutoff)
        .exec(conn)
        .await?;

    if result.rows_affected == 0 {
        return Ok(None);
    }

    find_by_id(conn, submission_id).await
}

fn claimable_submission_query(
    submission_id: i64,
    now: i64,
    cutoff: i64,
) -> sea_orm::UpdateMany<Entity> {
    Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Processing),
            processing_at: Set(Some(now)),
            checked_at: Set(None),
            ..Default::default()
        })
        .filter(Column::Id.eq(submission_id))
        .filter(
            Condition::any().add(Column::Status.eq(Status::Queued)).add(
                Condition::all()
                    .add(Column::Status.eq(Status::Processing))
                    .add(
                        Condition::any()
                            .add(Column::ProcessingAt.is_null())
                            .add(Column::ProcessingAt.lte(cutoff)),
                    ),
            ),
        )
}

/// Returns a claimed submission to the queue after a transient checker failure.
pub async fn release_processing(
    conn: &impl ConnectionTrait,
    submission_id: i64,
    processing_at: i64,
) -> Result<bool, DbError> {
    let result = Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Queued),
            processing_at: Set(None),
            checked_at: Set(None),
            ..Default::default()
        })
        .filter(Column::Id.eq(submission_id))
        .filter(Column::Status.eq(Status::Processing))
        .filter(Column::ProcessingAt.eq(processing_at))
        .exec(conn)
        .await?;

    Ok(result.rows_affected == 1)
}

/// Stores a final status only when the submission is still owned by a checker.
pub async fn finish_processing(
    conn: &impl ConnectionTrait,
    submission_id: i64,
    processing_at: i64,
    status: Status,
) -> Result<Option<SubmissionView>, DbError> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let result = Entity::update_many()
        .set(ActiveModel {
            status: Set(status),
            checked_at: Set(Some(now)),
            ..Default::default()
        })
        .filter(Column::Id.eq(submission_id))
        .filter(Column::Status.eq(Status::Processing))
        .filter(Column::ProcessingAt.eq(processing_at))
        .exec(conn)
        .await?;

    if result.rows_affected == 0 {
        return Ok(None);
    }

    find_by_id(conn, submission_id).await
}

/// Returns expired in-flight rows to the queue during startup recovery.
pub async fn reset_stale_processing(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    let cutoff = time::OffsetDateTime::now_utc()
        .unix_timestamp()
        .saturating_sub(PROCESSING_LEASE_SECONDS);

    Ok(stale_processing_reset_query(cutoff)
        .exec(conn)
        .await?
        .rows_affected)
}

fn stale_processing_reset_query(cutoff: i64) -> sea_orm::UpdateMany<Entity> {
    Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Queued),
            processing_at: Set(None),
            checked_at: Set(None),
            ..Default::default()
        })
        .filter(Column::Status.eq(Status::Processing))
        .filter(
            Condition::any()
                .add(Column::ProcessingAt.is_null())
                .add(Column::ProcessingAt.lte(cutoff)),
        )
}

/// Looks up correct by team ids and game id.

pub async fn find_correct_by_team_ids_and_game_id(
    conn: &impl ConnectionTrait,
    team_ids: Vec<i64>,
    game_id: i64,
) -> Result<Vec<SubmissionView>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::TeamId.is_in(team_ids))
        .filter(Column::GameId.eq(game_id))
        .filter(Column::Status.eq(Status::Correct))
        .all(conn)
        .await?
        .into_iter()
        .map(SubmissionView::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

/// Looks up correct by challenge ids and optional team game.

pub async fn find_correct_by_challenge_ids_and_optional_team_game(
    conn: &impl ConnectionTrait,
    challenge_ids: Vec<i64>,
    team_id: Option<i64>,
    game_id: Option<i64>,
) -> Result<Vec<SubmissionView>, DbError> {
    let mut loader = Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::ChallengeId.is_in(challenge_ids));

    if let (Some(_), Some(game_id)) = (team_id, game_id) {
        loader = loader.filter(Column::GameId.eq(game_id));
    } else {
        loader = loader
            .filter(Column::GameId.is_null())
            .filter(Column::TeamId.is_null());
    }

    let submissions = loader
        .filter(Column::Status.eq(Status::Correct))
        .order_by_asc(Column::CreatedAt)
        .all(conn)
        .await?
        .into_iter()
        .map(SubmissionView::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(submissions)
}

/// Checks if the given team+game scope has a `Cheat` submission for the
/// specified challenge.
pub async fn has_cheat(
    conn: &impl ConnectionTrait,
    challenge_id: i64,
    team_id: i64,
    game_id: i64,
) -> Result<bool, DbError> {
    let (submissions, _) = find(
        conn,
        FindSubmissionsOptions {
            challenge_id: Some(challenge_id),
            team_id: Some(Some(team_id)),
            game_id: Some(Some(game_id)),
            status: Some(Status::Cheat),
            page: Some(1),
            size: Some(1),
            ..Default::default()
        },
    )
    .await?;

    Ok(!submissions.is_empty())
}

/// Returns challenge_ids where the team has a `Cheat` submission, filtered to
/// the given set of candidate challenge_ids.
pub async fn find_cheat_challenge_ids(
    conn: &impl ConnectionTrait,
    challenge_ids: Vec<i64>,
    team_id: i64,
    game_id: i64,
) -> Result<Vec<i64>, DbError> {
    let submissions = Entity::find()
        .filter(Column::ChallengeId.is_in(challenge_ids))
        .filter(Column::Status.eq(Status::Cheat))
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::GameId.eq(game_id))
        .select_only()
        .column(Column::ChallengeId)
        .into_tuple::<i64>()
        .all(conn)
        .await?;

    Ok(submissions)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Counts submissions in `Correct` status for the given scope.
pub async fn count_correct(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::Status.eq(Status::Correct))
        .count(conn)
        .await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<SubmissionView, DbError> {
    let submission = model.insert(conn).await?;
    info!(
        submission_id = submission.id,
        user_id = submission.user_id,
        team_id = submission.team_id,
        game_id = submission.game_id,
        challenge_id = submission.challenge_id,
        status = ?submission.status,
        "submission created"
    );

    Ok(find_by_id(conn, submission.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("submission_{}", submission.id)))?)
}

/// Applies an active model update to the database.
pub async fn update(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<SubmissionView, DbError> {
    let submission = model.update(conn).await?;
    info!(
        submission_id = submission.id,
        user_id = submission.user_id,
        team_id = submission.team_id,
        game_id = submission.game_id,
        challenge_id = submission.challenge_id,
        status = ?submission.status,
        pts = submission.pts,
        rank = submission.rank,
        "submission updated"
    );

    Ok(find_by_id(conn, submission.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("submission_{}", submission.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, submission_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(submission_id).exec(conn).await?;
    info!(submission_id, "submission deleted");

    Ok(())
}

#[cfg(test)]
mod score_tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::*;

    #[test]
    fn score_updates_are_chunked_and_built_with_bound_values() {
        let updates = (0..super::super::BULK_UPDATE_BATCH_SIZE + 1)
            .map(|id| ScoreUpdate {
                id: id as i64,
                pts: 100,
                rank: id as i64 + 1,
            })
            .collect::<Vec<_>>();

        let statements = score_update_statements(7, &updates);
        assert_eq!(statements.len(), 2);

        let first = DbBackend::Postgres.build(&statements[0]);
        let second = DbBackend::Postgres.build(&statements[1]);
        assert_eq!(
            first.values.unwrap().0.len(),
            super::super::BULK_UPDATE_BATCH_SIZE * 4
        );
        assert_eq!(second.values.unwrap().0.len(), 4);
        assert!(first.sql.starts_with("UPDATE \"submissions\""));
        assert!(first.sql.contains("FROM (VALUES"));
        assert!(first.sql.contains("\"submission_scores\".\"column3\""));
    }

    #[test]
    fn empty_score_updates_build_no_statements() {
        assert!(score_update_statements(1, &[]).is_empty());
    }

    #[test]
    fn score_input_query_keeps_partial_index_predicate_literal() {
        let statement = score_input_query(7).build(DbBackend::Postgres);

        assert!(statement.sql.contains("\"status\" = 'correct'"));
        assert_eq!(statement.values.unwrap().0.len(), 1);
    }

    #[test]
    fn stale_processing_reset_uses_the_fifteen_second_lease_cutoff() {
        let cutoff = 1_000 - PROCESSING_LEASE_SECONDS;
        let statement = stale_processing_reset_query(cutoff).build(DbBackend::Postgres);

        assert!(statement.sql.starts_with("UPDATE \"submissions\""));
        assert!(statement.sql.contains("\"status\" = $4"));
        assert!(statement.sql.contains("\"processing_at\" IS NULL"));
        assert!(statement.sql.contains("\"processing_at\" <= $5"));
        assert_eq!(
            statement.values.unwrap().0,
            vec![
                Status::Queued.into(),
                Option::<i64>::None.into(),
                Option::<i64>::None.into(),
                Status::Processing.into(),
                985_i64.into(),
            ]
        );
        assert_eq!(PROCESSING_LEASE_SECONDS, 15);
    }

    #[test]
    fn claim_query_accepts_only_queued_or_stale_processing_rows() {
        let statement = claimable_submission_query(7, 1_000, 990).build(DbBackend::Postgres);

        assert!(statement.sql.starts_with("UPDATE \"submissions\""));
        assert!(statement.sql.contains("\"id\" = $4"));
        assert!(statement.sql.contains("\"status\" = $5"));
        assert!(statement.sql.contains("\"status\" = $6"));
        assert!(statement.sql.contains("\"processing_at\" IS NULL"));
        assert!(statement.sql.contains("\"processing_at\" <= $7"));
        assert_eq!(
            statement.values.unwrap().0,
            vec![
                Status::Processing.into(),
                Some(1_000_i64).into(),
                Option::<i64>::None.into(),
                7_i64.into(),
                Status::Queued.into(),
                Status::Processing.into(),
                990_i64.into(),
            ]
        );
    }
}

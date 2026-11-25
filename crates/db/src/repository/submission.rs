//! Database access for `submission` — SeaORM queries, updates, and DTOs.
//!
//! # Judging queue contract
//!
//! PostgreSQL is the **single source of truth** for pending flag checks:
//! - Workers claim batches of `queued` or lease-expired `processing` rows
//!   atomically via [`claim_batch`] (`FOR UPDATE SKIP LOCKED`), excluding
//!   challenges that are already at their local per-script concurrency cap.
//! - Each claim increments the monotonic `claims` fencing counter. Terminal
//!   writes ([`finish_processing`]) and transient rollbacks
//!   ([`release_processing`]) require `status = 'processing' AND claims = $c`
//!   so a stalled worker can never overwrite a newer attempt.
//! - At-most-one `correct` solve per `(game_id, team_id, challenge_id)` or
//!   standalone `(user_id, challenge_id)` is enforced directly by PostgreSQL
//!   partial unique indexes (`uq_submissions_correct_*`), eliminating
//!   application-level advisory locks.

use std::{collections::HashMap, str::FromStr};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityLoaderTrait, EntityTrait,
    FromQueryResult, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait, Set, SqlErr, Statement, Value,
    sea_query::{Expr, ExprTrait},
};

pub(crate) use crate::entity::submission::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::{
        scoreboard::ScoreboardSubmission,
        submission::{SubmissionSummary, SubmissionView},
    },
    entity::submission::{ActiveModel, Model as SubmissionRecord, Status},
};

/// Processing lease duration before another worker may take over a stalled
/// `processing` row. Must remain strictly greater than the worker's per-check
/// timeout (`10s`).
pub const PROCESSING_LEASE_SECONDS: i64 = 15;

/// Minimal submission row returned by [`claim_batch`].
///
/// Carries everything the checker and finalizer need without joining `users`,
/// `challenges`, `teams`, or `games`.
#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
pub struct ClaimedSubmission {
    pub id: i64,
    pub content: String,
    pub challenge_id: i64,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub created_at: time::OffsetDateTime,
    pub claims: i64,
}

/// Outcome of a fenced terminal write via [`finish_processing`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FinishResult {
    /// Row transitioned from `processing` to the requested terminal status.
    Committed,
    /// Another worker or administrator superseded this `claims` generation.
    LeaseLost,
    /// PostgreSQL rejected `status = 'correct'` because the partial unique
    /// index (`uq_submissions_correct_*`) already holds a solve in this scope.
    DuplicateCorrect,
}

/// Narrow submission projection used to build a game's CQRS score snapshot.
#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
pub struct ScoreInput {
    pub id: i64,
    pub challenge_id: i64,
    pub team_id: i64,
    pub created_at: time::OffsetDateTime,
}

/// Aggregated solve statistics for one standalone (practice) challenge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandaloneChallengeStats {
    pub solved: bool,
    pub solved_times: i64,
    pub blood_ids: Vec<i64>,
}

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
            claims: submission.claims,
            checked_at: submission.checked_at,
            pts: 0,
            rank: 0,
        })
    }
}

/// Projects a SeaORM 2 loaded submission graph to the public scoreboard
/// contract. Derived `pts` defaults to `0` and is hydrated by the score
/// service.
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
            pts: 0,
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

/// Atomically claims up to `limit` queued or lease-expired submissions using
/// `FOR UPDATE SKIP LOCKED`, skipping any challenges listed in
/// `exclude_challenge_ids`.
///
/// The query hits `idx_submissions_work_queue` (`WHERE status IN ('queued',
/// 'processing')`) and returns only the columns required for checking and
/// finalization.
pub async fn claim_batch(
    conn: &impl ConnectionTrait,
    limit: usize,
    exclude_challenge_ids: &[i64],
) -> Result<Vec<ClaimedSubmission>, DbError> {
    if limit == 0 {
        return Ok(Vec::new());
    }
    let now = time::OffsetDateTime::now_utc();
    let lease_cutoff = now - time::Duration::seconds(PROCESSING_LEASE_SECONDS);
    let statement = claim_batch_statement(now, lease_cutoff, limit as i64, exclude_challenge_ids);

    Ok(ClaimedSubmission::find_by_statement(statement)
        .all(conn)
        .await?)
}

fn claim_batch_statement(
    now: time::OffsetDateTime,
    lease_cutoff: time::OffsetDateTime,
    limit: i64,
    exclude_challenge_ids: &[i64],
) -> Statement {
    let excluded: Value = exclude_challenge_ids.to_vec().into();
    Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            WITH candidate AS (
                SELECT id
                FROM submissions
                WHERE (
                    status = 'queued'
                    OR (
                        status = 'processing'
                        AND (processing_at IS NULL OR processing_at <= $1)
                    )
                )
                  AND (cardinality($2::bigint[]) = 0 OR NOT (challenge_id = ANY($2::bigint[])))
                ORDER BY created_at ASC, id ASC
                LIMIT $3
                FOR UPDATE SKIP LOCKED
            )
            UPDATE submissions AS s
            SET status = 'processing',
                processing_at = $4,
                claims = s.claims + 1,
                checked_at = NULL
            FROM candidate AS c
            WHERE s.id = c.id
            RETURNING
                s.id,
                s.content,
                s.challenge_id,
                s.user_id,
                s.team_id,
                s.game_id,
                s.created_at,
                s.claims
        "#,
        [lease_cutoff.into(), excluded, limit.into(), now.into()],
    )
}

/// Returns a claimed submission back to `queued` after a transient error (e.g.
/// checker VM capacity exhaustion) if this worker still owns `claims`.
pub async fn release_processing(
    conn: &impl ConnectionTrait,
    submission_id: i64,
    claims: i64,
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
        .filter(Column::Claims.eq(claims))
        .exec(conn)
        .await?;

    Ok(result.rows_affected == 1)
}

/// Persists a terminal status only when `claims` still matches the active
/// processing lease.
///
/// Maps PostgreSQL unique-constraint violations (`uq_submissions_correct_*`)
/// into [`FinishResult::DuplicateCorrect`] so callers can demote concurrent
/// duplicate solves without holding advisory locks.
pub async fn finish_processing(
    conn: &impl ConnectionTrait,
    submission_id: i64,
    claims: i64,
    status: Status,
) -> Result<FinishResult, DbError> {
    let now = time::OffsetDateTime::now_utc();
    let result = Entity::update_many()
        .set(ActiveModel {
            status: Set(status),
            checked_at: Set(Some(now)),
            ..Default::default()
        })
        .filter(Column::Id.eq(submission_id))
        .filter(Column::Status.eq(Status::Processing))
        .filter(Column::Claims.eq(claims))
        .exec(conn)
        .await;

    match result {
        Ok(res) if res.rows_affected == 1 => Ok(FinishResult::Committed),
        Ok(_) => Ok(FinishResult::LeaseLost),
        Err(err) if matches!(err.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            Ok(FinishResult::DuplicateCorrect)
        }
        Err(err) => Err(err.into()),
    }
}

/// Advances the claim generation for an administrator-initiated processing
/// transition so any in-flight worker holding the previous generation is
/// fenced out.
pub async fn advance_claim_generation(
    conn: &impl ConnectionTrait,
    submission_id: i64,
) -> Result<(), DbError> {
    let result = Entity::update_many()
        .col_expr(Column::Claims, Expr::col(Column::Claims).add(1_i64))
        .filter(Column::Id.eq(submission_id))
        .exec(conn)
        .await?;

    if result.rows_affected == 0 {
        return Err(DbError::NotFound(format!("submission_{submission_id}")));
    }

    Ok(())
}

/// Loads all valid (`Passed` team) correct submissions for a game using only
/// the fields required to build the CQRS score snapshot.
///
/// Joining `teams` on `state = Passed` ensures banned or unapproved teams never
/// distort dynamic challenge decay or blood bonuses.
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
        ])
        .join(
            JoinType::InnerJoin,
            crate::entity::submission::Relation::Team.def(),
        )
        .filter(Column::GameId.eq(game_id))
        // Keep the partial-index predicate constant so PostgreSQL always picks
        // `idx_submissions_scoring` even under generic prepared plans.
        .filter(Expr::col((Entity, Column::Status)).eq(Expr::Constant("correct".into())))
        .filter(crate::entity::team::Column::State.eq(crate::entity::team::State::Passed))
        .order_by_asc(Column::ChallengeId)
        .order_by_asc(Column::CreatedAt)
        .order_by_asc(Column::Id)
}

/// Batch-loads display views for a bounded set of submission IDs (e.g. top-3
/// bloods per challenge), preserving the caller's ordering.
pub async fn find_views_by_ids(
    conn: &impl ConnectionTrait,
    ids: &[i64],
) -> Result<Vec<SubmissionView>, DbError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let models = Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::Id.is_in(ids.iter().copied()))
        .all(conn)
        .await?;

    let mut by_id: HashMap<i64, SubmissionView> = HashMap::with_capacity(models.len());
    for model in models {
        let view = SubmissionView::try_from(model)?;
        by_id.insert(view.id, view);
    }

    Ok(ids.iter().filter_map(|id| by_id.remove(id)).collect())
}

/// Computes per-challenge solve count, user solved flag, and top-3 blood
/// submission IDs for standalone (non-game) challenges directly in SQL.
///
/// Avoids scanning and joining every historical solve into application memory.
pub async fn find_standalone_challenge_stats(
    conn: &impl ConnectionTrait,
    challenge_ids: &[i64],
    user_id: i64,
) -> Result<HashMap<i64, StandaloneChallengeStats>, DbError> {
    if challenge_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let ids_val: Value = challenge_ids.to_vec().into();

    #[derive(FromQueryResult)]
    struct AggregateRow {
        challenge_id: i64,
        solved_times: i64,
        solved: bool,
    }

    let aggregates = AggregateRow::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            SELECT
                challenge_id,
                COUNT(*)::bigint AS solved_times,
                COALESCE(BOOL_OR(user_id = $1), FALSE) AS solved
            FROM submissions
            WHERE game_id IS NULL
              AND team_id IS NULL
              AND status = 'correct'
              AND challenge_id = ANY($2::bigint[])
            GROUP BY challenge_id
        "#,
        [user_id.into(), ids_val.clone()],
    ))
    .all(conn)
    .await?;

    #[derive(FromQueryResult)]
    struct BloodRow {
        challenge_id: i64,
        id: i64,
    }

    let blood_rows = BloodRow::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            SELECT challenge_id, id
            FROM (
                SELECT
                    challenge_id,
                    id,
                    ROW_NUMBER() OVER (
                        PARTITION BY challenge_id
                        ORDER BY created_at ASC, id ASC
                    ) AS rn
                FROM submissions
                WHERE game_id IS NULL
                  AND team_id IS NULL
                  AND status = 'correct'
                  AND challenge_id = ANY($1::bigint[])
            ) ranked
            WHERE rn <= 3
            ORDER BY challenge_id ASC, rn ASC
        "#,
        [ids_val],
    ))
    .all(conn)
    .await?;

    let mut stats: HashMap<i64, StandaloneChallengeStats> = aggregates
        .into_iter()
        .map(|row| {
            (
                row.challenge_id,
                StandaloneChallengeStats {
                    solved: row.solved,
                    solved_times: row.solved_times,
                    blood_ids: Vec::with_capacity(3),
                },
            )
        })
        .collect();

    for row in blood_rows {
        if let Some(entry) = stats.get_mut(&row.challenge_id) {
            entry.blood_ids.push(row.id);
        }
    }

    Ok(stats)
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

/// Counts distinct challenges solved by a user, grouped by challenge category.
pub async fn count_correct_by_user_and_category(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Vec<(i32, u64)>, DbError> {
    let counts = Entity::find()
        .select_only()
        .column(crate::entity::challenge::Column::Category)
        .column_as(Expr::col(Column::ChallengeId).count_distinct(), "solved")
        .join(
            JoinType::InnerJoin,
            crate::entity::submission::Relation::Challenge.def(),
        )
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Status.eq(Status::Correct))
        .group_by(crate::entity::challenge::Column::Category)
        .order_by_asc(crate::entity::challenge::Column::Category)
        // PostgreSQL COUNT returns BIGINT; sqlx cannot decode it as u64.
        .into_tuple::<(i32, i64)>()
        .all(conn)
        .await?;

    Ok(counts
        .into_iter()
        .map(|(category, solved)| (category, solved as u64))
        .collect())
}

/// Counts distinct challenges a user solved in one game.
pub async fn count_correct_by_user_and_game(
    conn: &impl ConnectionTrait,
    user_id: i64,
    game_id: i64,
) -> Result<u64, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::ChallengeId)
        .filter(Column::UserId.eq(user_id))
        .filter(Column::GameId.eq(game_id))
        .filter(Column::Status.eq(Status::Correct))
        .distinct()
        .count(conn)
        .await?)
}

/// Checks if the given team+game scope has a `Cheat` submission for the
/// specified challenge.
pub async fn has_cheat(
    conn: &impl ConnectionTrait,
    challenge_id: i64,
    team_id: i64,
    game_id: i64,
) -> Result<bool, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .filter(Column::ChallengeId.eq(challenge_id))
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::GameId.eq(game_id))
        // Keep the partial-index predicate constant; scope IDs remain bound.
        .filter(Expr::col(Column::Status).eq(Expr::Constant(Status::Cheat.into())))
        .into_tuple::<i64>()
        .one(conn)
        .await?
        .is_some())
}

/// Returns challenge_ids where the team has a `Cheat` submission, filtered to
/// the given set of candidate challenge_ids.
pub async fn find_cheat_challenge_ids(
    conn: &impl ConnectionTrait,
    challenge_ids: Vec<i64>,
    team_id: i64,
    game_id: i64,
) -> Result<Vec<i64>, DbError> {
    if challenge_ids.is_empty() {
        return Ok(Vec::new());
    }
    let submissions = Entity::find()
        .filter(Column::ChallengeId.is_in(challenge_ids))
        .filter(Expr::col(Column::Status).eq(Expr::Constant(Status::Cheat.into())))
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::GameId.eq(game_id))
        .select_only()
        .column(Column::ChallengeId)
        .into_tuple::<i64>()
        .all(conn)
        .await?;

    Ok(submissions)
}

/// Counts all submission rows.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Counts submissions in `Correct` status.
pub async fn count_correct(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::Status.eq(Status::Correct))
        .count(conn)
        .await?)
}

/// Inserts a submission without loading its user, challenge, team, or game.
/// Callers that already validated those relations can build an admission
/// response without fallible reads after this write has committed.
pub async fn create_record(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<SubmissionRecord, DbError> {
    let submission = model.insert(conn).await?;
    tracing::info!(
        resource = "submission",
        action = "create",
        submission_id = submission.id,
        user_id = submission.user_id,
        team_id = submission.team_id,
        game_id = submission.game_id,
        challenge_id = submission.challenge_id,
        status = ?submission.status,
        "database step completed",
    );

    Ok(submission)
}

/// Inserts a new row and loads its associated display information.
pub async fn create(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<SubmissionView, DbError> {
    let submission = create_record(conn, model).await?;
    find_by_id(conn, submission.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("submission_{}", submission.id)))
}

/// Applies an active model update to the database.
///
/// Maps PostgreSQL unique-constraint violations (`uq_submissions_correct_*`)
/// into [`DbError::Conflict`] with `"correct_submission_already_exists"`.
pub async fn update(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<SubmissionView, DbError> {
    let changed_fields = [
        ("content", model.content.is_set()),
        ("status", model.status.is_set()),
        ("challenge_id", model.challenge_id.is_set()),
        ("user_id", model.user_id.is_set()),
        ("team_id", model.team_id.is_set()),
        ("game_id", model.game_id.is_set()),
        ("processing_at", model.processing_at.is_set()),
        ("claims", model.claims.is_set()),
        ("checked_at", model.checked_at.is_set()),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect::<Vec<_>>();
    let submission = match model.update(conn).await {
        Ok(submission) => submission,
        Err(err) if matches!(err.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            return Err(DbError::Conflict(
                "correct_submission_already_exists".to_owned(),
            ));
        }
        Err(err) => return Err(err.into()),
    };
    tracing::info!(
        resource = "submission",
        action = "update",
        changed_fields = ?changed_fields,
        submission_id = submission.id,
        user_id = submission.user_id,
        team_id = submission.team_id,
        game_id = submission.game_id,
        challenge_id = submission.challenge_id,
        status = ?submission.status,
        "database step completed",
    );

    Ok(find_by_id(conn, submission.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("submission_{}", submission.id)))?)
}

/// Deletes rows matching the provided identifier.
pub async fn delete(conn: &impl ConnectionTrait, submission_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(submission_id).exec(conn).await?;
    tracing::info!(
        resource = "submission",
        action = "delete",
        submission_id = submission_id,
        "resource step completed"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::*;

    #[test]
    fn score_input_query_filters_passed_teams_and_keeps_partial_index_literal() {
        let statement = score_input_query(7).build(DbBackend::Postgres);

        assert!(statement.sql.contains("INNER JOIN \"teams\""));
        assert!(
            statement
                .sql
                .contains("\"submissions\".\"status\" = 'correct'")
        );
        assert!(statement.sql.contains("\"teams\".\"state\" = $2"));
        assert_eq!(statement.values.unwrap().0.len(), 2);
    }

    #[test]
    fn claim_batch_uses_skip_locked_and_challenge_exclusions() {
        let now = time::OffsetDateTime::from_unix_timestamp(1_000).unwrap();
        let cutoff =
            time::OffsetDateTime::from_unix_timestamp(1_000 - PROCESSING_LEASE_SECONDS).unwrap();
        let statement = claim_batch_statement(now, cutoff, 8, &[10, 20]);

        assert!(statement.sql.contains("FOR UPDATE SKIP LOCKED"));
        assert!(statement.sql.contains("challenge_id = ANY($2::bigint[])"));
        assert!(statement.sql.contains("claims = s.claims + 1"));
        assert_eq!(PROCESSING_LEASE_SECONDS, 15);
    }
}

//! JetStream consumer for subject **`cds.submission.check`**: resolves
//! **queued** flag submissions with the Lua [`cds_checker::Checker`], applies
//! game rules (duplicate, freeze, cheat), and may enqueue [`crate::calculator`]
//! work when a submission becomes correct.
//!
//! # Message format
//!
//! Each job is a plain string decimal: the submission **database id** (`i64`).
//!
//! # Startup
//!
//! [`spawn`] returns expired `Processing` rows to `Queued` and
//! re-publishes every queued submission so no job is lost after a restart.

use std::{future::Future, sync::Arc, time::Duration};

use anyhow::anyhow;
use cds_checker::Checker;
use cds_db::{
    DB, GameChallengeView, GameDetail, SubmissionView, TeamView, UserAccountView,
    sea_orm::{ActiveValue::Unchanged, IntoActiveModel, Set, TransactionTrait},
    submission::{FindSubmissionsOptions, Status},
    team::{Model, State},
};
use cds_queue::Queue;
use futures_util::StreamExt as _;
use tracing::{debug, error, info, warn};

use crate::calculator;

/// JetStream subject for asynchronous submission verification.
pub const SUBJECT: &str = "cds.submission.check";

/// Maximum number of submissions checked concurrently by this process.
const MAX_IN_FLIGHT: usize = 16;

/// Maximum wall-clock time allowed for one checker invocation.
const CHECK_TIMEOUT: Duration = Duration::from_secs(10);

async fn enforce_check_timeout<F: Future>(
    future: F,
) -> Result<F::Output, tokio::time::error::Elapsed> {
    tokio::time::timeout(CHECK_TIMEOUT, future).await
}

/// Shared handles for one consumer instance (cloned into async jobs).
#[derive(Clone)]
struct Context {
    db: DB,
    queue: Queue,
    checker: Checker,
}

impl Context {
    /// Clones all dependencies into an owned [`Context`].
    fn new(db: &DB, queue: &Queue, checker: &Checker) -> Self {
        Self {
            db: db.clone(),
            queue: queue.clone(),
            checker: checker.clone(),
        }
    }
}

/// Loads a [`GameDetail`] row or fails with `game_not_found`.
#[tracing::instrument(skip_all, fields(game_id = game_id))]
async fn prepare_game(db: &cds_db::DB, game_id: i64) -> Result<GameDetail, anyhow::Error> {
    cds_db::game::find_by_id(&db.conn, game_id)
        .await?
        .ok_or_else(|| anyhow!("game_not_found"))
}

/// Loads the join row between a game and a challenge
/// (`game_challenge_not_found` on miss).
#[tracing::instrument(skip_all, fields(game_id = game_id, challenge_id = challenge_id))]
async fn prepare_game_challenge(
    db: &cds_db::DB,
    game_id: i64,
    challenge_id: i64,
) -> Result<GameChallengeView, anyhow::Error> {
    cds_db::game_challenge::find_by_id(&db.conn, game_id, challenge_id)
        .await?
        .ok_or_else(|| anyhow!("game_challenge_not_found"))
}

/// Runs a claimed submission, applies game rules, and notifies the calculator.
#[tracing::instrument(skip_all, fields(submission_id = submission.id))]
async fn check(
    ctx: Arc<Context>,
    submission: SubmissionView,
    processing_at: i64,
) -> Result<(), anyhow::Error> {
    let user = if let Some(user) =
        cds_db::user::find_by_id::<UserAccountView>(&ctx.db.conn, submission.user_id).await?
    {
        user
    } else {
        cds_db::submission::delete(&ctx.db.conn, submission.id).await?;
        return Err(anyhow!("user_not_found"));
    };

    let challenge = if let Some(challenge) =
        cds_db::challenge::find_by_id(&ctx.db.conn, submission.challenge_id).await?
    {
        challenge
    } else {
        cds_db::submission::delete(&ctx.db.conn, submission.id).await?;
        return Err(anyhow!("challenge_not_found"));
    };

    // Checker scripts key dynamic data off team id when present, otherwise the
    // submitting user.
    let operator_id = match submission.team_id {
        Some(team_id) => team_id,
        _ => submission.user_id,
    };

    let checker_result = enforce_check_timeout(ctx.checker.check(
        &challenge,
        operator_id,
        &submission.content,
    ))
    .await;
    let mut status = match checker_result {
        Ok(Ok(c_status)) => match c_status {
            cds_checker::Status::Correct => Status::Correct,
            cds_checker::Status::Incorrect => Status::Incorrect,
            cds_checker::Status::Cheat(peer_team_id) => {
                handle_cheat(ctx.clone(), &submission, peer_team_id)
                    .await
                    .unwrap_or_else(|_| Status::Incorrect)
            }
        },
        Ok(Err(err)) => {
            warn!(
                submission_id = submission.id,
                challenge_id = challenge.id,
                error = ?err,
                "checker script failed"
            );
            Status::Incorrect
        }
        Err(_) => {
            warn!(
                submission_id = submission.id,
                challenge_id = challenge.id,
                timeout_seconds = CHECK_TIMEOUT.as_secs(),
                "checker invocation timed out"
            );
            Status::Incorrect
        }
    };

    if status == Status::Correct {
        // Second (or later) correct flag for the same challenge scope becomes
        // Duplicate.
        let is_already_correct =
            if let (Some(game_id), Some(team_id)) = (submission.game_id, submission.team_id) {
                cds_db::submission::find(
                    &ctx.db.conn,
                    FindSubmissionsOptions {
                        challenge_id: Some(submission.challenge_id),
                        game_id: Some(Some(game_id)),
                        team_id: Some(Some(team_id)),
                        status: Some(Status::Correct),
                        ..Default::default()
                    },
                )
                .await?
                .1 > 0
            } else {
                cds_db::submission::find(
                    &ctx.db.conn,
                    FindSubmissionsOptions {
                        challenge_id: Some(submission.challenge_id),
                        user_id: Some(submission.user_id),
                        status: Some(Status::Correct),
                        team_id: Some(None),
                        game_id: Some(None),
                        ..Default::default()
                    },
                )
                .await?
                .1 > 0
            };

        if is_already_correct {
            status = Status::Duplicate;
        }

        if let (Some(game_id), Some(_team_id)) = (submission.game_id, submission.team_id) {
            let game = prepare_game(&ctx.db, game_id).await?;
            let game_challenge = prepare_game_challenge(&ctx.db, game_id, challenge.id).await?;

            // Late solves after global or per-challenge freeze windows downgrade to
            // Expired.
            let now = time::OffsetDateTime::now_utc().unix_timestamp();
            if now > game.frozen_at || now > game.ended_at {
                status = Status::Expired;
            }
            if let Some(frozen_at) = game_challenge.frozen_at {
                if now > frozen_at {
                    status = Status::Expired;
                }
            }
        }
    }

    info!(
        submission_id = submission.id,
        status = ?status,
        user_id = user.id,
        username = %user.username,
        challenge_id = challenge.id,
        game_id = submission.game_id,
        team_id = submission.team_id,
        "submission checked"
    );

    let transaction = ctx.db.conn.begin().await?;
    let Some(submission) = cds_db::submission::finish_processing(
        &transaction,
        submission.id,
        processing_at,
        status.clone(),
    )
    .await?
    else {
        warn!(
            submission_id = submission.id,
            "submission status changed while checker was running"
        );
        return Ok(());
    };

    let score_game_id = if let (Some(game_id), Status::Correct) = (submission.game_id, &status) {
        cds_db::game::request_score_recalculation(&transaction, game_id).await?;
        Some(game_id)
    } else {
        None
    };
    transaction.commit().await?;

    if let Some(game_id) = score_game_id {
        // Fan-out score recompute for the affected competition only.
        calculator::notify(&ctx.queue, game_id).await;
        debug!(
            submission_id = submission.id,
            game_id,
            subject = calculator::SUBJECT,
            "score recalculation queued"
        );
    }

    Ok(())
}

/// Marks both the submitting team and the `peer_team_id` as **Banned** when
/// cheat is detected.
#[tracing::instrument(skip_all, fields(
    submission_id = submission.id,
    game_id = submission.game_id,
    team_id = submission.team_id,
    peer_team_id = peer_team_id
))]
async fn handle_cheat(
    ctx: Arc<Context>,
    submission: &SubmissionView,
    peer_team_id: i64,
) -> Result<Status, anyhow::Error> {
    let (Some(game_id), Some(team_id)) = (submission.game_id, submission.team_id) else {
        return Ok(Status::Incorrect);
    };

    if let (Some(team), Some(peer_team)) = (
        cds_db::team::find_by_id::<Model>(&ctx.db.conn, team_id, game_id).await?,
        cds_db::team::find_by_id::<Model>(&ctx.db.conn, peer_team_id, game_id).await?,
    ) {
        let transaction = ctx.db.conn.begin().await?;
        for t in &[team, peer_team] {
            warn!(
                team_id = t.id,
                game_id, peer_team_id, "team banned by cheat detection"
            );
            let _ = cds_db::team::update::<TeamView>(
                &transaction,
                cds_db::team::ActiveModel {
                    id: Unchanged(t.id),
                    state: Set(State::Banned),
                    ..t.clone().into_active_model()
                },
            )
            .await?;
        }
        cds_db::game::request_score_recalculation(&transaction, game_id).await?;
        transaction.commit().await?;
        calculator::notify(&ctx.queue, game_id).await;
    }

    Ok(Status::Cheat)
}

/// Re-publishes historical `Queued` rows so they are checked after deploys /
/// crashes.
#[tracing::instrument(skip_all)]
async fn recover_queued(ctx: Arc<Context>) -> Result<(), anyhow::Error> {
    let reset = cds_db::submission::reset_stale_processing(&ctx.db.conn).await?;
    let (unchecked_submissions, _) = cds_db::submission::find(
        &ctx.db.conn,
        FindSubmissionsOptions {
            status: Some(Status::Queued),
            sorts: Some("created_at".to_owned()),
            ..Default::default()
        },
    )
    .await?;

    let recovered = unchecked_submissions.len();
    for submission in unchecked_submissions {
        let id = submission.id;
        ctx.queue.publish(SUBJECT, id).await?;
    }

    info!(
        count = recovered,
        reset,
        subject = SUBJECT,
        "queued submissions recovered"
    );

    Ok(())
}

/// Pulls messages and runs at most [`MAX_IN_FLIGHT`] checker jobs concurrently.
#[tracing::instrument(skip_all, fields(subject = SUBJECT))]
async fn run(ctx: Arc<Context>) -> Result<(), anyhow::Error> {
    let messages = ctx.queue.subscribe(SUBJECT, None).await?;
    messages
        .for_each_concurrent(Some(MAX_IN_FLIGHT), |result| {
            let ctx = Arc::clone(&ctx);
            async move {
                let message = match result {
                    Ok(message) => message,
                    Err(err) => {
                        error!(error = ?err, "checker message receive failed");
                        return;
                    }
                };

                let id = match serde_json::from_slice::<i64>(&message.payload) {
                    Ok(id) => id,
                    Err(err) => {
                        warn!(error = ?err, "invalid checker payload skipped");
                        message.double_ack().await.ok();
                        return;
                    }
                };
                debug!(submission_id = id, "checker message received");

                let submission =
                    match cds_db::submission::claim_queued_by_id(&ctx.db.conn, id).await {
                        Ok(Some(submission)) => submission,
                        Ok(None) => {
                            debug!(
                                submission_id = id,
                                "submission is not queued; message skipped"
                            );
                            message.double_ack().await.ok();
                            return;
                        }
                        Err(err) => {
                            error!(submission_id = id, error = ?err, "submission claim failed");
                            return;
                        }
                    };
                let Some(processing_at) = submission.processing_at else {
                    error!(
                        submission_id = id,
                        "claimed submission has no processing timestamp"
                    );
                    return;
                };
                debug!(
                    submission_id = submission.id,
                    user_id = submission.user_id,
                    team_id = submission.team_id,
                    game_id = submission.game_id,
                    challenge_id = submission.challenge_id,
                    processing_at,
                    "submission claimed"
                );

                let mut acknowledge = true;
                if let Err(err) = check(Arc::clone(&ctx), submission, processing_at).await {
                    let released =
                        cds_db::submission::release_processing(&ctx.db.conn, id, processing_at)
                            .await
                            .unwrap_or(false);
                    acknowledge = !released;
                    error!(submission_id = id, released, error = ?err, "submission check failed");
                }

                if acknowledge {
                    message.double_ack().await.ok();
                }
            }
        })
        .await;

    Ok(())
}

/// Starts the consumer task after calling [`recover_queued`].
#[tracing::instrument(skip_all, fields(handler = "spawn"))]
pub async fn spawn(db: &DB, queue: &Queue, checker: &Checker) {
    let ctx = Arc::new(Context::new(db, queue, checker));
    recover_queued(Arc::clone(&ctx)).await.unwrap();

    let run_ctx = Arc::clone(&ctx);
    tokio::spawn(async move {
        if let Err(err) = run(run_ctx).await {
            error!("{:?}", err);
        }
    });

    info!(
        subject = SUBJECT,
        concurrency = MAX_IN_FLIGHT,
        "queue consumer spawned"
    );
}

#[cfg(test)]
mod tests {
    use std::future::pending;

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn checker_deadline_expires_after_ten_seconds() {
        let started_at = tokio::time::Instant::now();
        assert!(enforce_check_timeout(pending::<()>()).await.is_err());
        assert_eq!(started_at.elapsed(), Duration::from_secs(10));
    }
}

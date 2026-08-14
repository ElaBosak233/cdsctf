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

mod finalizer;

use std::{future::Future, sync::Arc, time::Duration};

use anyhow::anyhow;
use cds_checker::Checker;
use cds_db::{
    DB, SubmissionView, UserAccountView,
    submission::{FindSubmissionsOptions, PROCESSING_LEASE_SECONDS, Status},
};
use cds_queue::{Queue, async_nats::jetstream::AckKind};
use futures_util::StreamExt as _;
use tracing::{debug, error, info, warn};

use self::finalizer::{FinalizeOutcome, Verdict};
use crate::calculator;

/// JetStream subject for asynchronous submission verification.
pub const SUBJECT: &str = "cds.submission.check";

/// Maximum number of submissions checked concurrently by this process.
const MAX_IN_FLIGHT: usize = 16;

/// Maximum wall-clock time allowed for one checker invocation.
const CHECK_TIMEOUT: Duration = Duration::from_secs(10);

/// Redelivers an unacknowledged checker message shortly after its database
/// processing lease expires.
const CHECKER_ACK_WAIT: Duration = Duration::from_secs(PROCESSING_LEASE_SECONDS as u64 + 1);

const TRANSIENT_RETRY_DELAY: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CheckOutcome {
    Committed,
    LeaseLost,
}

async fn enforce_check_timeout<F: Future>(
    future: F,
) -> Result<F::Output, tokio::time::error::Elapsed> {
    tokio::time::timeout(CHECK_TIMEOUT, future).await
}

fn processing_retry_after_at(processing_at: Option<i64>, now: i64) -> Duration {
    let expires_at = processing_at
        .unwrap_or(now)
        .saturating_add(PROCESSING_LEASE_SECONDS);
    Duration::from_secs(expires_at.saturating_sub(now).max(1) as u64)
}

fn processing_retry_after(processing_at: Option<i64>) -> Duration {
    processing_retry_after_at(
        processing_at,
        time::OffsetDateTime::now_utc().unix_timestamp(),
    )
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

/// Runs a claimed submission, applies game rules, and notifies the calculator.
#[tracing::instrument(skip_all, fields(submission_id = submission.id))]
async fn check(
    ctx: &Context,
    submission: SubmissionView,
    processing_at: i64,
) -> Result<CheckOutcome, anyhow::Error> {
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
    let verdict = match checker_result {
        Ok(Ok(c_status)) => match c_status {
            cds_checker::Status::Correct => Verdict::Correct,
            cds_checker::Status::Incorrect => Verdict::Incorrect,
            cds_checker::Status::Cheat(peer_team_id) => Verdict::Cheat { peer_team_id },
        },
        Ok(Err(err)) => {
            warn!(
                submission_id = submission.id,
                challenge_id = challenge.id,
                error = ?err,
                "checker script failed"
            );
            Verdict::Incorrect
        }
        Err(_) => {
            warn!(
                submission_id = submission.id,
                challenge_id = challenge.id,
                timeout_seconds = CHECK_TIMEOUT.as_secs(),
                "checker invocation timed out"
            );
            Verdict::Incorrect
        }
    };

    let FinalizeOutcome::Committed {
        status,
        score_game_id,
    } = finalizer::finalize(&ctx.db, &submission, processing_at, verdict).await?
    else {
        warn!(
            submission_id = submission.id,
            "submission status changed while checker was running"
        );
        return Ok(CheckOutcome::LeaseLost);
    };

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

    if let (Status::Cheat, Verdict::Cheat { peer_team_id }, Some(game_id)) =
        (&status, verdict, score_game_id)
    {
        warn!(
            submission_id = submission.id,
            game_id,
            team_id = submission.team_id,
            peer_team_id,
            "teams banned by cheat detection"
        );
    }

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

    Ok(CheckOutcome::Committed)
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
    let messages = ctx
        .queue
        .subscribe_with_ack_wait(SUBJECT, None, CHECKER_ACK_WAIT)
        .await?;
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
                    match cds_db::submission::claim_queued_or_stale_by_id(&ctx.db.conn, id).await {
                        Ok(Some(submission)) => submission,
                        Ok(None) => {
                            match cds_db::submission::find_by_id(&ctx.db.conn, id).await {
                                Ok(Some(submission))
                                    if matches!(submission.status, Status::Queued) =>
                                {
                                    debug!(submission_id = id, "queued submission claim raced");
                                    message
                                        .ack_with(AckKind::Nak(Some(TRANSIENT_RETRY_DELAY)))
                                        .await
                                        .ok();
                                }
                                Ok(Some(submission))
                                    if matches!(submission.status, Status::Processing) =>
                                {
                                    let retry_after =
                                        processing_retry_after(submission.processing_at);
                                    debug!(
                                        submission_id = id,
                                        processing_at = submission.processing_at,
                                        retry_after_ms = retry_after.as_millis(),
                                        "submission lease is active; message delayed"
                                    );
                                    message.ack_with(AckKind::Nak(Some(retry_after))).await.ok();
                                }
                                Ok(_) => {
                                    debug!(
                                        submission_id = id,
                                        "submission is terminal or missing; message skipped"
                                    );
                                    message.double_ack().await.ok();
                                }
                                Err(err) => {
                                    error!(
                                        submission_id = id,
                                        error = ?err,
                                        "submission state lookup failed"
                                    );
                                }
                            }
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

                match check(&ctx, submission, processing_at).await {
                    Ok(CheckOutcome::Committed) => {
                        message.double_ack().await.ok();
                    }
                    Ok(CheckOutcome::LeaseLost) => {
                        debug!(
                            submission_id = id,
                            "submission lease lost; message left unacknowledged"
                        );
                    }
                    Err(err) => {
                        let released = match cds_db::submission::release_processing(
                            &ctx.db.conn,
                            id,
                            processing_at,
                        )
                        .await
                        {
                            Ok(released) => released,
                            Err(release_err) => {
                                error!(
                                    submission_id = id,
                                    error = ?err,
                                    release_error = ?release_err,
                                    "submission check and lease release failed; message left unacknowledged"
                                );
                                return;
                            }
                        };
                        error!(submission_id = id, released, error = ?err, "submission check failed");
                        if released {
                            message
                                .ack_with(AckKind::Nak(Some(TRANSIENT_RETRY_DELAY)))
                                .await
                                .ok();
                        }
                    }
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

    #[test]
    fn processing_retry_waits_until_the_lease_expires() {
        assert_eq!(
            processing_retry_after_at(Some(90), 100),
            Duration::from_secs(5)
        );
        assert_eq!(
            processing_retry_after_at(Some(85), 100),
            Duration::from_secs(1)
        );
        assert_eq!(
            processing_retry_after_at(None, 100),
            Duration::from_secs(PROCESSING_LEASE_SECONDS as u64)
        );
    }

    #[test]
    fn checker_ack_wait_exceeds_the_processing_lease() {
        assert_eq!(CHECKER_ACK_WAIT, Duration::from_secs(16));
        assert!(CHECKER_ACK_WAIT.as_secs() > PROCESSING_LEASE_SECONDS as u64);
    }
}

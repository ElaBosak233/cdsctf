//! JetStream consumer for subject **`cds.submission.check`**: resolves
//! **queued** flag submissions with the Lua [`cds_checker::Checker`], applies
//! game rules (duplicate, freeze, cheat), and may enqueue [`crate::calculator`]
//! work when a submission becomes correct.
//!
//! # Message format
//!
//! Each job is a plain string decimal: the submission **database id** (`i64`).
//!
//! # Recovery
//!
//! [`spawn`] returns expired `Processing` rows to `Queued` and
//! re-publishes queued submissions. Periodic scans also recover persisted work
//! whose initial message publication failed, without restarting the
//! application.

mod dispatcher;
mod finalizer;
mod recovery;

use std::{future::Future, sync::Arc, time::Duration};

use anyhow::anyhow;
use cds_checker::Checker;
use cds_db::{
    DB, SubmissionView, UserAccountView,
    submission::{PROCESSING_LEASE_SECONDS, Status},
};
use cds_queue::Queue;
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

/// Only explicit Lua verdicts classify an answer. Admission errors are safe
/// to retry because no script code ran; other failures require operator review.
fn checker_verdict(
    result: Result<cds_checker::Status, cds_checker::traits::CheckerError>,
) -> Result<Verdict, cds_checker::traits::CheckerError> {
    match result {
        Ok(cds_checker::Status::Correct) => Ok(Verdict::Correct),
        Ok(cds_checker::Status::Incorrect) => Ok(Verdict::Incorrect),
        Ok(cds_checker::Status::Cheat(peer_team_id)) => Ok(Verdict::Cheat { peer_team_id }),
        Err(error) if error.is_busy() => Err(error),
        Err(error) => {
            warn!(?error, "checker script failed");
            Ok(Verdict::Failed)
        }
    }
}

async fn enforce_check_timeout<F: Future>(
    future: F,
) -> Result<F::Output, tokio::time::error::Elapsed> {
    tokio::time::timeout(CHECK_TIMEOUT, future).await
}

fn processing_retry_after_at(
    processing_at: Option<time::OffsetDateTime>,
    now: time::OffsetDateTime,
) -> Duration {
    let expires_at =
        processing_at.unwrap_or(now) + time::Duration::seconds(PROCESSING_LEASE_SECONDS);
    Duration::from_secs((expires_at - now).whole_seconds().max(1) as u64)
}

fn processing_retry_after(processing_at: Option<time::OffsetDateTime>) -> Duration {
    processing_retry_after_at(processing_at, time::OffsetDateTime::now_utc())
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
    processing_at: time::OffsetDateTime,
    claims: i64,
    initiator_id: Option<i64>,
    reservation: Option<cds_checker::Reservation>,
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

    let subject = match (submission.game_id, submission.team_id) {
        (Some(_), Some(team_id)) => cds_checker::Subject::Team(team_id),
        (None, None) => cds_checker::Subject::User(submission.user_id),
        _ => return Err(anyhow!("invalid_submission_scope")),
    };

    let checker_result = enforce_check_timeout(async {
        match reservation {
            Some(permit) => {
                ctx.checker
                    .check_reserved(&challenge, subject, &submission.content, permit)
                    .await
            }
            None => {
                ctx.checker
                    .check(&challenge, subject, &submission.content)
                    .await
            }
        }
    })
    .await;
    let verdict = match checker_result {
        Ok(result) => checker_verdict(result)?,
        Err(_) => {
            warn!(
                submission_id = submission.id,
                challenge_id = challenge.id,
                timeout_seconds = CHECK_TIMEOUT.as_secs(),
                "checker invocation timed out"
            );
            Verdict::Failed
        }
    };

    let FinalizeOutcome::Committed {
        status,
        score_game_id,
    } = finalizer::finalize(&ctx.db, &submission, processing_at, claims, verdict).await?
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
        calculator::notify(&ctx.queue, game_id, initiator_id).await;
        debug!(
            submission_id = submission.id,
            game_id,
            subject = calculator::SUBJECT,
            "score recalculation queued"
        );
    }

    Ok(CheckOutcome::Committed)
}

/// Starts the slot-first consumer and independent publication recovery tasks.
#[tracing::instrument(skip_all, fields(handler = "spawn"))]
pub async fn spawn(db: &DB, queue: &Queue, checker: &Checker) {
    let ctx = Arc::new(Context::new(db, queue, checker));
    let run_ctx = Arc::clone(&ctx);
    tokio::spawn(async move {
        loop {
            match dispatcher::run(Arc::clone(&run_ctx)).await {
                Err(error) => error!(?error, "checker consumer stopped; reconnecting"),
                Ok(()) => warn!("checker message stream ended; reconnecting"),
            }
            tokio::time::sleep(TRANSIENT_RETRY_DELAY).await;
        }
    });
    tokio::spawn(recovery::run(ctx));

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
    fn separates_checker_failures_from_wrong_answers_and_retries() {
        use cds_checker::traits::CheckerError;
        assert_eq!(
            checker_verdict(Ok(cds_checker::Status::Incorrect)).unwrap(),
            Verdict::Incorrect
        );
        assert_eq!(
            checker_verdict(Ok(cds_checker::Status::Correct)).unwrap(),
            Verdict::Correct
        );
        assert_eq!(
            checker_verdict(Err(CheckerError::ScriptError("broken checker".to_owned()))).unwrap(),
            Verdict::Failed
        );
        let busy = CheckerError::EngineError(cds_checker::traits::EngineError::Busy);
        assert!(checker_verdict(Err(busy)).unwrap_err().is_busy());
    }

    #[test]
    fn processing_retry_waits_until_the_lease_expires() {
        let now = time::OffsetDateTime::from_unix_timestamp(100).unwrap();
        assert_eq!(
            processing_retry_after_at(
                Some(time::OffsetDateTime::from_unix_timestamp(90).unwrap()),
                now
            ),
            Duration::from_secs(5)
        );
        assert_eq!(
            processing_retry_after_at(
                Some(time::OffsetDateTime::from_unix_timestamp(85).unwrap()),
                now
            ),
            Duration::from_secs(1)
        );
        assert_eq!(
            processing_retry_after_at(None, now),
            Duration::from_secs(PROCESSING_LEASE_SECONDS as u64)
        );
    }

    #[test]
    fn checker_ack_wait_exceeds_the_processing_lease() {
        assert_eq!(CHECKER_ACK_WAIT, Duration::from_secs(16));
        assert!(CHECKER_ACK_WAIT.as_secs() > PROCESSING_LEASE_SECONDS as u64);
    }
}

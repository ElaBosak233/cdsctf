//! Pull-on-capacity flag checker worker backed by PostgreSQL as the single
//! source of truth.
//!
//! # Architecture
//!
//! - **Single queue (PostgreSQL)**: Pending submissions live solely in
//!   `submissions` (`idx_submissions_work_queue`). Each worker pod claims at
//!   most `MAX_IN_FLIGHT - running` rows via `FOR UPDATE SKIP LOCKED`, passing
//!   its locally saturated `challenge_id`s in `challenge_id <> ALL($2)` so one
//!   slow challenge never starves other challenges or other pods.
//! - **Instant, zero-herd wakeups (NATS Core Queue Group)**: `POST
//!   /submissions` and admin requeues send a 0-byte NATS Core notification on
//!   [`WAKEUP_SUBJECT`]. All workers share queue group [`WAKEUP_QUEUE_GROUP`],
//!   so each submission wakes exactly one worker pod in sub-milliseconds.
//! - **Automatic recovery**: Crashed workers' expired `processing` leases and
//!   any dropped NATS wakeups are reclaimed by the exact same `SKIP LOCKED`
//!   query on a 2-second fallback tick — no separate recovery scanner needed.
//! - **Cache coherence**: Challenge scripts are cached in memory for 60s and
//!   evicted immediately across all pods via [`CHALLENGE_INVALIDATE_SUBJECT`].

mod finalizer;

use std::{
    collections::HashMap,
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};

use cds_checker::Checker;
use cds_db::{ChallengeDetail, DB, submission::ClaimedSubmission};
use cds_queue::Queue;
use cds_score::ScoreService;
use dashmap::DashMap;
use futures_util::StreamExt as _;
use tokio::task::JoinSet;
use tracing::{Instrument, debug, error, info, warn};

use self::finalizer::{FinalizeOutcome, Verdict};

/// NATS Core subject used as a lightweight doorbell when new work is queued.
pub const WAKEUP_SUBJECT: &str = "cds.submission.wakeup";

/// NATS Core queue group so each wakeup signal activates only one worker pod.
const WAKEUP_QUEUE_GROUP: &str = "checker-workers";

/// NATS Core broadcast subject carrying a decimal `challenge_id` whose checker
/// script or key changed.
pub const CHALLENGE_INVALIDATE_SUBJECT: &str = "cds.checker.invalidated";

/// Maximum concurrent submission checks executed by one worker process.
const MAX_IN_FLIGHT: usize = 16;

/// Maximum concurrent checks for any single challenge within one worker
/// process (matches `cds-engine`'s per-script semaphore limit).
const MAX_PER_CHALLENGE: usize = 4;

/// Maximum wall-clock duration allowed for one Lua checker invocation.
const CHECK_TIMEOUT: Duration = Duration::from_secs(10);

/// Periodic fallback poll to self-heal expired leases or missed NATS signals.
const FALLBACK_POLL_INTERVAL: Duration = Duration::from_secs(2);

/// In-process TTL for cached `ChallengeDetail` scripts before re-reading PG.
const CHALLENGE_CACHE_TTL: Duration = Duration::from_secs(60);

/// Sends a best-effort 0-byte wakeup signal to one idle checker worker.
///
/// Safe to ignore failures: the submission is already committed in PostgreSQL
/// and will be picked up by the fallback poll within
/// [`FALLBACK_POLL_INTERVAL`].
pub async fn notify(queue: &Queue) {
    if let Err(error) = queue.notify(WAKEUP_SUBJECT, b"").await {
        warn!(
            ?error,
            "checker wakeup notification failed; fallback poll will recover"
        );
    }
}

/// Broadcasts a challenge script/key invalidation so every worker pod evicts
/// its cached checker immediately.
pub async fn notify_challenge_updated(queue: &Queue, challenge_id: i64) {
    if let Err(error) = queue
        .notify(CHALLENGE_INVALIDATE_SUBJECT, challenge_id.to_string())
        .await
    {
        warn!(
            challenge_id,
            ?error,
            "checker invalidation broadcast failed"
        );
    }
}

/// Only explicit Lua verdicts classify an answer. Admission (`Busy`) errors
/// are returned so the submission lease is released and retried; script faults
/// become terminal `Failed` for operator review.
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

struct WorkerContext {
    db: DB,
    queue: Queue,
    checker: Checker,
    score: ScoreService,
    challenge_cache: DashMap<i64, (Arc<ChallengeDetail>, Instant)>,
}

impl WorkerContext {
    async fn load_challenge(
        &self,
        challenge_id: i64,
    ) -> Result<Option<Arc<ChallengeDetail>>, anyhow::Error> {
        if let Some(entry) = self.challenge_cache.get(&challenge_id)
            && entry.1.elapsed() < CHALLENGE_CACHE_TTL
        {
            return Ok(Some(Arc::clone(&entry.0)));
        }

        let Some(challenge) = cds_db::challenge::find_by_id(&self.db.conn, challenge_id).await?
        else {
            self.challenge_cache.remove(&challenge_id);
            return Ok(None);
        };
        let challenge = Arc::new(challenge);
        self.challenge_cache
            .insert(challenge_id, (Arc::clone(&challenge), Instant::now()));
        Ok(Some(challenge))
    }
}

/// Executes one claimed submission, commits its terminal verdict, and
/// invalidates the game's CQRS score epoch when scores changed.
async fn process_claimed(ctx: &WorkerContext, submission: ClaimedSubmission) {
    let submission_id = submission.id;
    let challenge_id = submission.challenge_id;
    let claims = submission.claims;

    let result: Result<(), anyhow::Error> = async {
        let Some(challenge) = ctx.load_challenge(challenge_id).await? else {
            cds_db::submission::delete(&ctx.db.conn, submission_id).await?;
            warn!(
                submission_id,
                challenge_id, "challenge missing; submission deleted"
            );
            return Ok(());
        };

        let subject = match (submission.game_id, submission.team_id) {
            (Some(_), Some(team_id)) => cds_checker::Subject::Team(team_id),
            (None, None) => cds_checker::Subject::User(submission.user_id),
            _ => {
                warn!(submission_id, "invalid submission scope; marking failed");
                let _ = finalizer::finalize(&ctx.db, &submission, Verdict::Failed).await?;
                return Ok(());
            }
        };

        let verdict = match enforce_check_timeout(ctx.checker.check(
            &challenge,
            subject,
            &submission.content,
        ))
        .await
        {
            Ok(result) => checker_verdict(result)?,
            Err(_) => {
                warn!(
                    submission_id,
                    challenge_id,
                    timeout_seconds = CHECK_TIMEOUT.as_secs(),
                    "checker invocation timed out"
                );
                Verdict::Failed
            }
        };

        match finalizer::finalize(&ctx.db, &submission, verdict).await? {
            FinalizeOutcome::Committed {
                status,
                score_game_id,
            } => {
                info!(
                    submission_id,
                    status = ?status,
                    user_id = submission.user_id,
                    challenge_id,
                    game_id = submission.game_id,
                    team_id = submission.team_id,
                    "submission checked"
                );
                if let (
                    cds_db::submission::Status::Cheat,
                    Verdict::Cheat { peer_team_id },
                    Some(game_id),
                ) = (&status, verdict, score_game_id)
                {
                    warn!(
                        submission_id,
                        game_id,
                        team_id = submission.team_id,
                        peer_team_id,
                        "teams banned by cheat detection"
                    );
                }
                if let Some(game_id) = score_game_id {
                    ctx.score.invalidate(game_id).await;
                }
            }
            FinalizeOutcome::LeaseLost => {
                warn!(
                    submission_id,
                    "submission lease changed while checker was running"
                );
            }
        }
        Ok(())
    }
    .await;

    if let Err(error) = result {
        let is_busy = error
            .downcast_ref::<cds_checker::traits::CheckerError>()
            .is_some_and(|e| e.is_busy());
        match cds_db::submission::release_processing(&ctx.db.conn, submission_id, claims).await {
            Ok(released) => {
                if is_busy {
                    debug!(
                        submission_id,
                        released, "checker capacity busy; submission released back to queue"
                    );
                } else {
                    error!(
                        submission_id,
                        released,
                        ?error,
                        "submission check failed transiently; released back to queue"
                    );
                }
            }
            Err(release_err) => {
                error!(
                    submission_id,
                    ?error,
                    ?release_err,
                    "submission check and lease release failed; lease will expire automatically"
                );
            }
        }
    }
}

/// Core pull-on-capacity loop. Only claims rows when local slots are free.
async fn run_loop(ctx: Arc<WorkerContext>) {
    let mut in_flight: JoinSet<(i64, i64)> = JoinSet::new();
    let mut per_challenge: HashMap<i64, usize> = HashMap::new();
    let mut wakeup = match ctx
        .queue
        .queue_subscribe(WAKEUP_SUBJECT, WAKEUP_QUEUE_GROUP)
        .await
    {
        Ok(sub) => Some(sub),
        Err(error) => {
            warn!(
                ?error,
                "checker wakeup subscribe failed; running with fallback polling"
            );
            None
        }
    };
    let mut fallback_tick = tokio::time::interval(FALLBACK_POLL_INTERVAL);
    fallback_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        // Fill available local capacity before waiting.
        while in_flight.len() < MAX_IN_FLIGHT {
            let free_slots = MAX_IN_FLIGHT - in_flight.len();
            let busy_challenges: Vec<i64> = per_challenge
                .iter()
                .filter_map(|(&cid, &count)| (count >= MAX_PER_CHALLENGE).then_some(cid))
                .collect();

            match cds_db::submission::claim_batch(&ctx.db.conn, free_slots, &busy_challenges).await
            {
                Ok(claimed) if claimed.is_empty() => break,
                Ok(claimed) => {
                    let count = claimed.len();
                    for submission in claimed {
                        let submission_id = submission.id;
                        let challenge_id = submission.challenge_id;
                        *per_challenge.entry(challenge_id).or_default() += 1;
                        let job_ctx = Arc::clone(&ctx);
                        let span = tracing::info_span!(
                            parent: None,
                            "submission.check",
                            submission_id,
                            challenge_id,
                            operator_kind = "system"
                        );
                        in_flight.spawn(
                            async move {
                                process_claimed(&job_ctx, submission).await;
                                (submission_id, challenge_id)
                            }
                            .instrument(span),
                        );
                    }
                    // If we saturated this pod on a full batch, ping a peer pod
                    // so queued overflow drains across the cluster immediately.
                    if count == free_slots && in_flight.len() >= MAX_IN_FLIGHT {
                        notify(&ctx.queue).await;
                    }
                }
                Err(error) => {
                    error!(?error, "submission claim_batch failed");
                    break;
                }
            }
        }

        // Wait for either a task to free a slot, a NATS wakeup doorbell, or the
        // periodic self-healing tick.
        tokio::select! {
            completed = in_flight.join_next(), if !in_flight.is_empty() => {
                if let Some(Ok((_sid, cid))) = completed {
                    if let Some(count) = per_challenge.get_mut(&cid) {
                        *count = count.saturating_sub(1);
                        if *count == 0 {
                            per_challenge.remove(&cid);
                        }
                    }
                } else if let Some(Err(error)) = completed {
                    error!(?error, "checker worker task panicked");
                }
            }
            msg = async {
                match wakeup.as_mut() {
                    Some(sub) => sub.next().await,
                    None => std::future::pending().await,
                }
            } => {
                if msg.is_none() {
                    warn!("checker wakeup subscription ended; reconnecting");
                    wakeup = ctx
                        .queue
                        .queue_subscribe(WAKEUP_SUBJECT, WAKEUP_QUEUE_GROUP)
                        .await
                        .ok();
                }
            }
            _ = fallback_tick.tick() => {
                if wakeup.is_none() {
                    wakeup = ctx
                        .queue
                        .queue_subscribe(WAKEUP_SUBJECT, WAKEUP_QUEUE_GROUP)
                        .await
                        .ok();
                }
            }
        }
    }
}

/// Listens for challenge script/key invalidation broadcasts so edited scripts
/// take effect immediately across all worker pods.
async fn run_invalidation_listener(ctx: Arc<WorkerContext>) {
    loop {
        match ctx.queue.subscribe_core(CHALLENGE_INVALIDATE_SUBJECT).await {
            Ok(mut sub) => {
                while let Some(msg) = sub.next().await {
                    if let Ok(text) = std::str::from_utf8(&msg.payload)
                        && let Ok(challenge_id) = text.trim().parse::<i64>()
                    {
                        ctx.challenge_cache.remove(&challenge_id);
                        ctx.checker.invalidate(challenge_id);
                        debug!(challenge_id, "evicted cached checker script and key");
                    }
                }
            }
            Err(error) => {
                warn!(?error, "checker invalidation subscribe failed; retrying");
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

/// Spawns the pull-on-capacity checker worker and its script invalidation
/// listener.
#[tracing::instrument(skip_all, fields(handler = "spawn"))]
pub async fn spawn(db: &DB, queue: &Queue, checker: &Checker, score: &ScoreService) {
    let ctx = Arc::new(WorkerContext {
        db: db.clone(),
        queue: queue.clone(),
        checker: checker.clone(),
        score: score.clone(),
        challenge_cache: DashMap::new(),
    });
    tokio::spawn(run_loop(Arc::clone(&ctx)));
    tokio::spawn(run_invalidation_listener(ctx));

    info!(
        wakeup_subject = WAKEUP_SUBJECT,
        concurrency = MAX_IN_FLIGHT,
        per_challenge_cap = MAX_PER_CHALLENGE,
        "checker worker spawned"
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
}

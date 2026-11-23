//! Slot-first checker dispatcher.
//!
//! Incoming submission messages are buffered per challenge in a bounded visible
//! window and dispatched round-robin only after reserving real script, domain,
//! and Lua-state permits from [`cds_checker::Checker`]. Waiting submissions
//! receive JetStream `Progress` acknowledgements instead of acquiring and
//! releasing database processing leases.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
    time::Duration,
};

use anyhow::anyhow;
use cds_db::submission::{PROCESSING_LEASE_SECONDS, Status};
use cds_queue::{
    Queue,
    async_nats::jetstream::{AckKind, Message},
};
use futures_util::StreamExt as _;
use tokio::task::JoinSet;
use tracing::{Instrument, debug, error, info, warn};

use super::{
    CHECKER_ACK_WAIT, CheckOutcome, Context, MAX_IN_FLIGHT, SUBJECT, TRANSIENT_RETRY_DELAY, check,
    processing_retry_after,
};

/// Upper bound on distinct submission IDs held in memory (queued + running).
const VISIBLE_LIMIT: usize = 4096;

/// Interval for extending JetStream ack deadlines on locally queued work.
const PROGRESS_INTERVAL: Duration = Duration::from_secs(4);

/// Fallback tick to re-check permits freed outside checker jobs (e.g.
/// `generate`).
const CAPACITY_TICK: Duration = Duration::from_millis(100);

struct Pending {
    id: i64,
    message: Message,
    challenge: cds_db::ChallengeDetail,
}

/// Bounded per-challenge FIFO queues with round-robin challenge rotation.
struct ChallengeQueues<T> {
    queues: HashMap<i64, VecDeque<T>>,
    rotation: VecDeque<i64>,
    visible: HashSet<i64>,
}

impl<T> ChallengeQueues<T> {
    fn new() -> Self {
        Self {
            queues: HashMap::new(),
            rotation: VecDeque::new(),
            visible: HashSet::new(),
        }
    }

    fn visible_len(&self) -> usize {
        self.visible.len()
    }

    fn contains(&self, id: i64) -> bool {
        self.visible.contains(&id)
    }

    fn challenge_count(&self) -> usize {
        self.queues.len()
    }

    fn rotation_len(&self) -> usize {
        self.rotation.len()
    }

    fn push(&mut self, id: i64, challenge_id: i64, item: T) {
        self.visible.insert(id);
        if !self.queues.contains_key(&challenge_id) {
            self.rotation.push_back(challenge_id);
        }
        self.queues.entry(challenge_id).or_default().push_back(item);
    }

    fn finish(&mut self, id: i64) {
        self.visible.remove(&id);
    }

    fn iter_pending(&self) -> impl Iterator<Item = &T> {
        self.queues.values().flat_map(|q| q.iter())
    }

    /// Pops the next challenge ID in round-robin order and returns a reference
    /// to its head item along with the challenge ID.
    fn pop_turn(&mut self) -> Option<i64> {
        self.rotation.pop_front()
    }

    fn front(&self, challenge_id: i64) -> Option<&T> {
        self.queues.get(&challenge_id).and_then(VecDeque::front)
    }

    fn pop_front(&mut self, challenge_id: i64) -> Option<T> {
        self.queues
            .get_mut(&challenge_id)
            .and_then(VecDeque::pop_front)
    }

    /// Completes a rotation turn: drops empty challenge buckets, otherwise
    /// returns the challenge to the back of the rotation cursor.
    fn end_turn(&mut self, challenge_id: i64) {
        let empty = self
            .queues
            .get(&challenge_id)
            .is_none_or(VecDeque::is_empty);
        if empty {
            self.queues.remove(&challenge_id);
        } else {
            self.rotation.push_back(challenge_id);
        }
    }
}

async fn handle_unclaimed_submission(ctx: &Context, id: i64, message: &Message) {
    match cds_db::submission::find_by_id(&ctx.db.conn, id).await {
        Ok(Some(submission)) if matches!(submission.status, Status::Queued) => {
            info!(
                submission_id = id,
                outcome = "deferred",
                "queued submission claim raced"
            );
            message
                .ack_with(AckKind::Nak(Some(TRANSIENT_RETRY_DELAY)))
                .await
                .ok();
        }
        Ok(Some(submission)) if matches!(submission.status, Status::Processing) => {
            let retry_after = processing_retry_after(submission.processing_at);
            info!(
                submission_id = id,
                outcome = "deferred",
                processing_at = ?submission.processing_at,
                retry_after_ms = retry_after.as_millis(),
                "submission lease is active; message delayed"
            );
            message.ack_with(AckKind::Nak(Some(retry_after))).await.ok();
        }
        Ok(_) => {
            info!(
                submission_id = id,
                outcome = "skipped",
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
}

/// Pulls messages and dispatches at most [`MAX_IN_FLIGHT`] checker jobs
/// concurrently after reserving execution capacity.
#[tracing::instrument(skip_all, fields(subject = SUBJECT))]
pub(super) async fn run(ctx: Arc<Context>) -> Result<(), anyhow::Error> {
    let mut messages = ctx
        .queue
        .subscribe_with_ack_wait(SUBJECT, None, CHECKER_ACK_WAIT)
        .await?;
    let mut queues: ChallengeQueues<Pending> = ChallengeQueues::new();
    let mut jobs: JoinSet<i64> = JoinSet::new();
    let mut progress = tokio::time::interval(PROGRESS_INTERVAL);
    let mut capacity_tick = tokio::time::interval(CAPACITY_TICK);

    loop {
        // Try at most one turn per visible challenge. Busy challenges keep
        // their head item in memory without claiming a database row.
        let turns = queues.rotation_len();
        let mut dispatched = false;
        for _ in 0..turns {
            let Some(cid) = queues.pop_turn() else {
                break;
            };
            if jobs.len() < MAX_IN_FLIGHT
                && let Some(front) = queues.front(cid)
            {
                let reservation = match ctx.checker.reserve(&front.challenge).await {
                    Ok(reservation) => Some(Some(reservation)),
                    Err(error) if error.is_busy() => None,
                    // Let the normal checker path classify script/preload errors as Failed.
                    Err(_) => Some(None),
                };
                if let Some(reservation) = reservation {
                    dispatched = true;
                    let pending = queues
                        .pop_front(cid)
                        .expect("front item must exist after reserve");
                    let ctx = Arc::clone(&ctx);
                    jobs.spawn(async move {
                        let id = pending.id;
                        let message = pending.message;
                        let span = Queue::message_span(&message);
                        let process = async {
                            tracing::Span::current().record("submission_id", id);
                            let submission = match cds_db::submission::claim_queued_or_stale_by_id(
                                &ctx.db.conn,
                                id,
                            )
                            .await
                            {
                                Ok(Some(submission)) => submission,
                                Ok(None) => {
                                    drop(reservation);
                                    handle_unclaimed_submission(&ctx, id, &message).await;
                                    return;
                                }
                                Err(err) => {
                                    drop(reservation);
                                    error!(submission_id = id, error = ?err, "submission claim failed");
                                    return;
                                }
                            };
                            let Some(processing_at) = submission.processing_at else {
                                drop(reservation);
                                error!(
                                    submission_id = id,
                                    "claimed submission has no processing timestamp"
                                );
                                return;
                            };
                            let claims = submission.claims;
                            debug!(
                                submission_id = submission.id,
                                user_id = submission.user_id,
                                team_id = submission.team_id,
                                game_id = submission.game_id,
                                challenge_id = submission.challenge_id,
                                processing_at = ?processing_at,
                                claims,
                                "submission claimed after slot reservation"
                            );
                            match check(
                                &ctx,
                                submission,
                                processing_at,
                                claims,
                                Queue::initiator_id(&message),
                                reservation,
                            )
                            .await
                            {
                                Ok(CheckOutcome::Committed) => {
                                    info!(
                                        submission_id = id,
                                        outcome = "succeeded",
                                        "submission check completed"
                                    );
                                    message.double_ack().await.ok();
                                }
                                Ok(CheckOutcome::LeaseLost) => {
                                    info!(
                                        submission_id = id,
                                        outcome = "deferred",
                                        "submission lease lost; message left unacknowledged"
                                    );
                                }
                                Err(err) => {
                                    let released = match cds_db::submission::release_processing(
                                        &ctx.db.conn,
                                        id,
                                        processing_at,
                                        claims,
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
                                    if err
                                        .downcast_ref::<cds_checker::traits::CheckerError>()
                                        .is_some_and(|error| error.is_busy())
                                    {
                                        info!(
                                            submission_id = id,
                                            released,
                                            "checker capacity exhausted; submission deferred"
                                        );
                                    } else {
                                        error!(
                                            submission_id = id,
                                            released,
                                            error = ?err,
                                            "submission check failed"
                                        );
                                    }
                                    if released {
                                        message
                                            .ack_with(AckKind::Nak(Some(TRANSIENT_RETRY_DELAY)))
                                            .await
                                            .ok();
                                    }
                                }
                            }
                        };
                        process.instrument(span).await;
                        id
                    });
                }
            }
            queues.end_turn(cid);
            // Advance cursor immediately after each dispatch to preserve
            // round-robin order.
            if dispatched {
                break;
            }
        }
        // Keep filling free permits without waiting for the next timer tick.
        if dispatched {
            continue;
        }

        tokio::select! {
            completed = jobs.join_next(), if !jobs.is_empty() => {
                match completed {
                    Some(Ok(id)) => {
                        queues.finish(id);
                    }
                    Some(Err(error)) => {
                        return Err(anyhow!("checker task join failed: {error}"));
                    }
                    None => {}
                }
            }
            item = messages.next(), if queues.visible_len() < VISIBLE_LIMIT => {
                let Some(item) = item else {
                    return Ok(());
                };
                let message = match item {
                    Ok(message) => message,
                    Err(err) => {
                        error!(error = ?err, "checker message receive failed");
                        continue;
                    }
                };
                let id = match serde_json::from_slice::<i64>(&message.payload) {
                    Ok(id) => id,
                    Err(err) => {
                        warn!(outcome = "rejected", error = ?err, "invalid checker payload skipped");
                        message.double_ack().await.ok();
                        continue;
                    }
                };
                info!(submission_id = id, "checker message received");
                if queues.contains(id) {
                    // Another delivery of work already buffered or running locally;
                    // the original delivery owns final acknowledgement.
                    message.ack_with(AckKind::Progress).await.ok();
                    continue;
                }
                let Some(submission) = cds_db::submission::find_by_id(&ctx.db.conn, id).await? else {
                    info!(submission_id = id, outcome = "skipped", "submission missing; message skipped");
                    message.double_ack().await.ok();
                    continue;
                };
                match submission.status {
                    Status::Queued => {}
                    Status::Processing
                        if submission.processing_at.is_some_and(|at| {
                            at + time::Duration::seconds(PROCESSING_LEASE_SECONDS)
                                <= time::OffsetDateTime::now_utc()
                        }) => {}
                    Status::Processing => {
                        let retry_after = processing_retry_after(submission.processing_at);
                        info!(
                            submission_id = id,
                            outcome = "deferred",
                            processing_at = ?submission.processing_at,
                            retry_after_ms = retry_after.as_millis(),
                            "submission lease is active; message delayed"
                        );
                        message.ack_with(AckKind::Nak(Some(retry_after))).await.ok();
                        continue;
                    }
                    _ => {
                        info!(
                            submission_id = id,
                            outcome = "skipped",
                            "submission is terminal; message skipped"
                        );
                        message.double_ack().await.ok();
                        continue;
                    }
                }
                let cid = submission.challenge_id;
                let Some(challenge) = cds_db::challenge::find_by_id(&ctx.db.conn, cid).await? else {
                    cds_db::submission::delete(&ctx.db.conn, id).await?;
                    message.double_ack().await.ok();
                    continue;
                };
                queues.push(id, cid, Pending { id, message, challenge });
            }
            _ = progress.tick() => {
                for pending in queues.iter_pending() {
                    if let Err(err) = pending.message.ack_with(AckKind::Progress).await {
                        warn!(submission_id = pending.id, ?err, "checker progress ack failed");
                    }
                }
                if queues.visible_len() > 0 {
                    debug!(
                        visible = queues.visible_len(),
                        running = jobs.len(),
                        challenges = queues.challenge_count(),
                        "checker dispatcher occupancy"
                    );
                }
            }
            _ = capacity_tick.tick() => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChallengeQueues;

    fn dispatch_one<F>(
        queues: &mut ChallengeQueues<i64>,
        mut can_reserve: F,
    ) -> Option<(i64, i64)>
    where
        F: FnMut(i64) -> bool, {
        let turns = queues.rotation_len();
        for _ in 0..turns {
            let cid = queues.pop_turn()?;
            let mut picked = None;
            if queues.front(cid).is_some() && can_reserve(cid) {
                picked = queues.pop_front(cid).map(|id| (cid, id));
            }
            queues.end_turn(cid);
            if picked.is_some() {
                return picked;
            }
        }
        None
    }

    #[test]
    fn round_robin_skips_busy_challenges_without_dropping_heads() {
        let mut q = ChallengeQueues::new();
        q.push(1, 10, 1);
        q.push(2, 10, 2);
        q.push(3, 20, 3);
        q.push(4, 20, 4);

        // Challenge 10 is busy -> skip it, dispatch challenge 20's head (3)
        // without claiming or popping challenge 10's head (1).
        assert_eq!(dispatch_one(&mut q, |cid| cid != 10), Some((20, 3)));
        assert_eq!(q.front(10), Some(&1));
        assert!(q.contains(1));

        // When both are busy, nothing is popped (zero database claims).
        assert_eq!(dispatch_one(&mut q, |_| false), None);
        assert_eq!(q.front(10), Some(&1));
        assert_eq!(q.front(20), Some(&4));

        // Once both have capacity, round-robin alternates cleanly and preserves
        // FIFO.
        assert_eq!(dispatch_one(&mut q, |_| true), Some((10, 1)));
        assert_eq!(dispatch_one(&mut q, |_| true), Some((20, 4)));
        assert_eq!(dispatch_one(&mut q, |_| true), Some((10, 2)));
        assert_eq!(dispatch_one(&mut q, |_| true), None);

        // Visible set tracks in-flight IDs until finish().
        assert!(q.contains(1));
        q.finish(1);
        assert!(!q.contains(1));
    }
}

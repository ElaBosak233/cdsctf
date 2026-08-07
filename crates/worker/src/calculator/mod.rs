//! Coalesced, transactional score recomputation for `cds.game.recalc`.
//!
//! The queue consumer runs at most one calculation per game locally, marks
//! messages that arrive during a run for one follow-up calculation, and runs
//! different games with bounded concurrency. PostgreSQL advisory transaction
//! locks extend the same-game exclusion across application instances.

mod math;
mod plan;
mod scheduler;

/// Defines the calculator queue payload.
pub mod payload;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use cds_db::{DB, game, game_challenge, sea_orm, submission, team};
use cds_queue::{Queue, async_nats::jetstream::Message};
use futures_util::StreamExt as _;
pub use payload::Payload;
use plan::ScorePlan;
use scheduler::{ScheduleAction, Scheduler};
use sea_orm::{ConnectionTrait, TransactionTrait};
use tokio::sync::{Semaphore, mpsc};
use tracing::{debug, error, info, warn};

/// JetStream subject name for score / rank recomputation jobs.
pub const SUBJECT: &str = "cds.game.recalc";

const MAX_PARALLEL_GAMES: usize = 4;
const DIRTY_RECONCILE_INTERVAL: Duration = Duration::from_secs(10);
const MAX_REVISION_PASSES: usize = 2;

type JobKey = Option<i64>;

fn needs_score_calculation(force: bool, revision: i64) -> bool {
    force || revision != 0
}

#[derive(Debug, Default)]
struct AppliedScores {
    revision: i64,
    caught_up: bool,
    submissions: u64,
    challenges: u64,
    teams: u64,
}

#[derive(Debug)]
struct JobResult {
    key: JobKey,
    result: Result<(), anyhow::Error>,
}

/// Rebuilds one game's score snapshot and persists it atomically.
#[tracing::instrument(skip_all, fields(game_id))]
async fn calculate(db: &DB, game_id: i64, mut force: bool) -> Result<(), anyhow::Error> {
    let mut passes = 0;
    loop {
        let started_at = Instant::now();
        let transaction = db.conn.begin().await?;

        let result = async {
            game::lock_score_recalculation(&transaction, game_id).await?;
            let Some(revision) = game::find_score_revision(&transaction, game_id).await? else {
                return Ok(None);
            };
            if !needs_score_calculation(force, revision) {
                return Ok(None);
            }

            let plan = load_score_plan(&transaction, game_id).await?;
            let mut applied = apply_score_plan(&transaction, game_id, plan).await?;
            applied.revision = revision;
            applied.caught_up =
                game::mark_score_recalculation_applied(&transaction, game_id, revision).await?;
            Ok(Some(applied))
        }
        .await;

        match result {
            Ok(None) => {
                transaction.commit().await?;
                debug!(
                    game_id,
                    "score calculation skipped; revision already applied"
                );
                return Ok(());
            }
            Ok(Some(applied)) => {
                transaction.commit().await?;
                passes += 1;
                info!(
                    game_id,
                    revision = applied.revision,
                    caught_up = applied.caught_up,
                    elapsed_ms = started_at.elapsed().as_millis(),
                    submissions = applied.submissions,
                    challenges = applied.challenges,
                    teams = applied.teams,
                    "score calculation completed"
                );
                if applied.caught_up {
                    return Ok(());
                }
                if passes >= MAX_REVISION_PASSES {
                    debug!(
                        game_id,
                        revision = applied.revision,
                        "score remains dirty after bounded follow-up"
                    );
                    return Ok(());
                }
                force = false;
            }
            Err(err) => {
                if let Err(rollback_err) = transaction.rollback().await {
                    error!(game_id, error = ?rollback_err, "score transaction rollback failed");
                }
                return Err(err);
            }
        }
    }
}

async fn load_score_plan(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<ScorePlan, anyhow::Error> {
    plan::build(
        submission::find_score_inputs(conn, game_id).await?,
        game_challenge::find_score_inputs(conn, game_id).await?,
        team::find_score_inputs(conn, game_id).await?,
    )
}

async fn apply_score_plan(
    conn: &impl ConnectionTrait,
    game_id: i64,
    plan: ScorePlan,
) -> Result<AppliedScores, anyhow::Error> {
    Ok(AppliedScores {
        revision: 0,
        caught_up: false,
        submissions: submission::update_scores(conn, game_id, &plan.submissions).await?,
        challenges: game_challenge::update_scores(conn, game_id, &plan.challenges).await?,
        teams: team::update_scores(conn, game_id, &plan.teams).await?,
    })
}

async fn calculate_scope(db: &DB, key: JobKey) -> Result<(), anyhow::Error> {
    if let Some(game_id) = key {
        return calculate(db, game_id, false).await;
    }

    let games = game::find_ids(&db.conn).await?;
    info!(games = games.len(), "calculator full rebuild requested");
    for game_id in games {
        calculate(db, game_id, true).await?;
    }
    Ok(())
}

fn spawn_calculation(
    db: DB,
    key: JobKey,
    semaphore: Arc<Semaphore>,
    completion_tx: mpsc::UnboundedSender<JobResult>,
) {
    tokio::spawn(async move {
        let result = match semaphore.acquire_owned().await {
            Ok(_permit) => calculate_scope(&db, key).await,
            Err(err) => Err(anyhow::anyhow!(err)),
        };
        completion_tx.send(JobResult { key, result }).ok();
    });
}

/// Dispatches queue messages without blocking ingestion while scores calculate.
#[tracing::instrument(skip_all, fields(subject = SUBJECT))]
async fn run(db: DB, queue: Queue) -> Result<(), anyhow::Error> {
    let mut messages = queue.subscribe(SUBJECT, None).await?;
    let mut scheduler = Scheduler::<JobKey, Option<Message>>::default();
    let semaphore = Arc::new(Semaphore::new(MAX_PARALLEL_GAMES));
    let (completion_tx, mut completion_rx) = mpsc::unbounded_channel::<JobResult>();
    let mut reconcile = tokio::time::interval(DIRTY_RECONCILE_INTERVAL);
    reconcile.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            maybe_message = messages.next() => {
                let Some(result) = maybe_message else {
                    return Ok(());
                };
                let message = match result {
                    Ok(message) => message,
                    Err(err) => {
                        error!(error = ?err, "calculator message receive failed");
                        continue;
                    }
                };
                let payload = match serde_json::from_slice::<Payload>(&message.payload) {
                    Ok(payload) => payload,
                    Err(err) => {
                        warn!(error = ?err, "invalid calculator payload skipped");
                        message.double_ack().await.ok();
                        continue;
                    }
                };
                let key = payload.game_id;
                debug!(game_id = key, "calculator message received");
                if scheduler.schedule(key, Some(message)) == ScheduleAction::Start {
                    spawn_calculation(
                        db.clone(),
                        key,
                        Arc::clone(&semaphore),
                        completion_tx.clone(),
                    );
                }
            }
            maybe_completion = completion_rx.recv() => {
                let Some(JobResult { key, result }) = maybe_completion else {
                    return Ok(());
                };
                let succeeded = result.is_ok();
                let Some(completion) = scheduler.complete(&key, succeeded) else {
                    warn!(game_id = key, "calculator completion had no scheduled job");
                    continue;
                };

                if let Err(err) = result {
                    error!(
                        game_id = key,
                        messages = completion.discarded.len(),
                        error = ?err,
                        "score calculation failed; messages left unacknowledged"
                    );
                } else {
                    for message in completion.acknowledged.into_iter().flatten() {
                        if let Err(err) = message.double_ack().await {
                            warn!(game_id = key, error = ?err, "calculator message ack failed");
                        }
                    }
                }

                if completion.rerun {
                    spawn_calculation(
                        db.clone(),
                        key,
                        Arc::clone(&semaphore),
                        completion_tx.clone(),
                    );
                }
            }
            _ = reconcile.tick() => {
                match game::find_dirty_score_ids(&db.conn).await {
                    Ok(game_ids) => {
                        for game_id in game_ids {
                            let key = Some(game_id);
                            if scheduler.schedule(key, None) == ScheduleAction::Start {
                                spawn_calculation(
                                    db.clone(),
                                    key,
                                    Arc::clone(&semaphore),
                                    completion_tx.clone(),
                                );
                            }
                        }
                    }
                    Err(err) => error!(error = ?err, "dirty score reconciliation failed"),
                }
            }
        }
    }
}

/// Marks a game dirty before publishing its calculator wake-up message.
pub async fn request(
    conn: &impl ConnectionTrait,
    queue: &Queue,
    game_id: i64,
) -> Result<(), anyhow::Error> {
    game::request_score_recalculation(conn, game_id).await?;
    notify(queue, game_id).await;
    Ok(())
}

/// Publishes a wake-up for a game already marked dirty in the database.
pub async fn notify(queue: &Queue, game_id: i64) {
    if let Err(err) = queue
        .publish(
            SUBJECT,
            Payload {
                game_id: Some(game_id),
            },
        )
        .await
    {
        warn!(game_id, error = ?err, "score wake-up publish failed; dirty scan will retry");
    }
}

/// Spawns the coalescing calculator dispatcher.
#[tracing::instrument(skip_all, fields(handler = "spawn"))]
pub async fn spawn(db: &DB, queue: &Queue) {
    let db = db.clone();
    let queue = queue.clone();
    tokio::spawn(async move {
        if let Err(err) = run(db, queue).await {
            error!(error = ?err, "calculator consumer stopped");
        }
    });

    info!(
        subject = SUBJECT,
        concurrency = MAX_PARALLEL_GAMES,
        "queue consumer spawned"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_revision_skips_duplicates_but_allows_forced_rebuilds() {
        assert!(!needs_score_calculation(false, 0));
        assert!(needs_score_calculation(false, 1));
        assert!(needs_score_calculation(true, 0));
        assert_eq!(MAX_REVISION_PASSES, 2);
    }
}

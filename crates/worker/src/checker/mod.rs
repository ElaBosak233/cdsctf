//! JetStream consumer for subject **`checker`**: resolves **pending** flag
//! submissions with the Rune [`cds_checker::Checker`], applies game rules
//! (duplicate, freeze, cheat), and may enqueue [`crate::calculator`] work when
//! a submission becomes correct.
//!
//! # Message format
//!
//! Each job is a plain string decimal: the submission **database id** (`i64`).
//!
//! # Startup
//!
//! [`spawn`] re-publishes every still-`Pending` submission so no job is lost
//! after a restart.

use std::sync::Arc;

use anyhow::anyhow;
use cds_checker::Checker;
use cds_db::{
    DB, Game, GameChallenge, Submission, Team, User,
    sea_orm::{ActiveValue::Unchanged, IntoActiveModel, Set},
    submission::{FindSubmissionsOptions, Status},
    team::{Model, State},
};
use cds_queue::Queue;
use futures_util::StreamExt as _;
use tracing::{error, info};

use crate::calculator::{self, Payload};

/// JetStream subject for asynchronous submission verification.
pub const SUBJECT: &str = "checker";

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

/// Loads a [`Game`] row or fails with `game_not_found`.
async fn prepare_game(db: &cds_db::DB, game_id: i64) -> Result<Game, anyhow::Error> {
    cds_db::game::find_by_id(&db.conn, game_id)
        .await?
        .ok_or_else(|| anyhow!("game_not_found"))
}

/// Loads the join row between a game and a challenge
/// (`game_challenge_not_found` on miss).
async fn prepare_game_challenge(
    db: &cds_db::DB,
    game_id: i64,
    challenge_id: i64,
) -> Result<GameChallenge, anyhow::Error> {
    cds_db::game_challenge::find_by_id(&db.conn, game_id, challenge_id)
        .await?
        .ok_or_else(|| anyhow!("game_challenge_not_found"))
}

/// End-to-end pipeline for one pending submission: load graph, run script,
/// post-process status, notify calculator.
async fn check(ctx: Arc<Context>, id: i64) -> Result<(), anyhow::Error> {
    let submission = cds_db::submission::find_pending_by_id::<Submission>(&ctx.db.conn, id)
        .await?
        .ok_or(anyhow!("submission_not_found"))?;

    let user = if let Some(user) =
        cds_db::user::find_by_id::<User>(&ctx.db.conn, submission.user_id).await?
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

    let mut status = match ctx
        .checker
        .check(&challenge, operator_id, &submission.content)
        .await
    {
        Ok(c_status) => match c_status {
            cds_checker::Status::Correct => Status::Correct,
            cds_checker::Status::Incorrect => Status::Incorrect,
            cds_checker::Status::Cheat(peer_team_id) => {
                handle_cheat(ctx.clone(), &submission, peer_team_id)
                    .await
                    .unwrap_or_else(|_| Status::Incorrect)
            }
        },
        Err(_) => Status::Incorrect,
    };

    if status == Status::Correct {
        // Second (or later) correct flag for the same challenge scope becomes
        // Duplicate.
        let is_already_correct =
            if let (Some(game_id), Some(team_id)) = (submission.game_id, submission.team_id) {
                cds_db::submission::find::<Submission>(
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
                cds_db::submission::find::<Submission>(
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
        "Submission #{}, status: {:?}, user: {}",
        submission.id, status, user.username
    );

    let submission = cds_db::submission::update::<Submission>(
        &ctx.db.conn,
        cds_db::submission::ActiveModel {
            id: Unchanged(submission.id),
            status: Set(status.clone()),
            ..Default::default()
        },
    )
    .await?;

    if submission.game_id.is_some() && status == Status::Correct {
        // Fan-out score recompute for the affected competition only.
        ctx.queue
            .publish(
                calculator::SUBJECT,
                Payload {
                    game_id: submission.game_id,
                },
            )
            .await?;
    }

    Ok(())
}

/// Marks both the submitting team and the `peer_team_id` as **Banned** when
/// cheat is detected.
async fn handle_cheat(
    ctx: Arc<Context>,
    submission: &Submission,
    peer_team_id: i64,
) -> Result<Status, anyhow::Error> {
    let (Some(game_id), Some(team_id)) = (submission.game_id, submission.team_id) else {
        return Ok(Status::Incorrect);
    };

    if let (Some(team), Some(peer_team)) = (
        cds_db::team::find_by_id::<Model>(&ctx.db.conn, team_id, game_id).await?,
        cds_db::team::find_by_id::<Model>(&ctx.db.conn, peer_team_id, game_id).await?,
    ) {
        for t in &[team, peer_team] {
            let _ = cds_db::team::update::<Team>(
                &ctx.db.conn,
                cds_db::team::ActiveModel {
                    id: Unchanged(t.id),
                    state: Set(State::Banned),
                    ..t.clone().into_active_model()
                },
            )
            .await?;
        }
    }

    Ok(Status::Cheat)
}

/// Re-queues historical `Pending` rows so they are checked after deploys /
/// crashes.
async fn recover_pending(ctx: Arc<Context>) -> Result<(), anyhow::Error> {
    let (unchecked_submissions, _) = cds_db::submission::find::<Submission>(
        &ctx.db.conn,
        FindSubmissionsOptions {
            status: Some(Status::Pending),
            sorts: Some("created_at".to_owned()),
            ..Default::default()
        },
    )
    .await?;

    for submission in unchecked_submissions {
        let id = submission.id;
        ctx.queue.publish(SUBJECT, id).await?;
    }

    Ok(())
}

/// Infinite pull loop: parse submission id, invoke [`check`], acknowledge the
/// JetStream message.
async fn run(ctx: Arc<Context>) -> Result<(), anyhow::Error> {
    let mut messages = ctx.queue.subscribe(SUBJECT, None).await?;
    while let Some(Ok(message)) = messages.next().await {
        let payload = String::from_utf8(message.payload.to_vec())?;
        let id = payload.parse::<i64>()?;

        if let Err(err) = check(Arc::clone(&ctx), id).await {
            error!("{:?}", err);
        }

        message.double_ack().await.ok();
    }

    Ok(())
}

/// Starts the consumer task and immediately calls [`recover_pending`].
pub async fn spawn(db: &DB, queue: &Queue, checker: &Checker) {
    let ctx = Arc::new(Context::new(db, queue, checker));
    let run_ctx = Arc::clone(&ctx);
    tokio::spawn(async move {
        if let Err(err) = run(run_ctx).await {
            error!("{:?}", err);
        }
    });

    recover_pending(ctx).await.unwrap();
    info!(subject = SUBJECT, "queue consumer spawned");
}

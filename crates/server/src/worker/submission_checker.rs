//! Checker module is for checking submissions,
//! it will assign a status to each submission.

use anyhow::anyhow;
use cds_db::{
    Submission, Team, User,
    sea_orm::{ActiveValue::Unchanged, IntoActiveModel, Set},
    submission::{FindSubmissionsOptions, Status},
    team::State,
};
use futures_util::StreamExt as _;
use tracing::{error, info};

use crate::worker::game_calculator;

async fn check(id: i64) -> Result<(), anyhow::Error> {
    let submission = cds_db::submission::find_pending_by_id::<Submission>(id)
        .await?
        .ok_or(anyhow!("submission_not_found"))?;

    let user = if let Some(user) = cds_db::user::find_by_id::<User>(submission.user_id).await? {
        user
    } else {
        cds_db::submission::delete(submission.id).await?;
        return Err(anyhow!("user_not_found"));
    };

    // Get related challenge
    let challenge =
        if let Some(challenge) = cds_db::challenge::find_by_id(submission.challenge_id).await? {
            challenge
        } else {
            cds_db::submission::delete(submission.id).await?;
            return Err(anyhow!("challenge_not_found"));
        };

    let operator_id = match submission.team_id {
        Some(team_id) => team_id,
        _ => submission.user_id,
    };

    let mut status = match cds_checker::check(&challenge, operator_id, &submission.content).await {
        Ok(c_status) => match c_status {
            cds_checker::Status::Correct => Status::Correct,
            cds_checker::Status::Incorrect => Status::Incorrect,
            cds_checker::Status::Cheat(peer_team_id) => handle_cheat(&submission, peer_team_id)
                .await
                .unwrap_or_else(|_| Status::Incorrect),
        },
        Err(_) => Status::Incorrect,
    };

    if status == Status::Correct {
        // Check whether the submission is duplicate.
        let is_already_correct =
            if let (Some(game_id), Some(team_id)) = (submission.game_id, submission.team_id) {
                cds_db::submission::find::<Submission>(FindSubmissionsOptions {
                    challenge_id: Some(submission.challenge_id),
                    game_id: Some(Some(game_id)),
                    team_id: Some(Some(team_id)),
                    status: Some(Status::Correct),
                    ..Default::default()
                })
                .await?
                .1 > 0
            } else {
                cds_db::submission::find::<Submission>(FindSubmissionsOptions {
                    challenge_id: Some(submission.challenge_id),
                    user_id: Some(submission.user_id),
                    status: Some(Status::Correct),
                    team_id: Some(None),
                    game_id: Some(None),
                    ..Default::default()
                })
                .await?
                .1 > 0
            };

        if is_already_correct {
            status = Status::Duplicate;
        }

        if let (Some(game_id), Some(_team_id)) = (submission.game_id, submission.team_id) {
            let game = crate::util::loader::prepare_game(game_id).await?;

            let game_challenge =
                crate::util::loader::prepare_game_challenge(game_id, challenge.id).await?;

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

    let submission = cds_db::submission::update::<Submission>(cds_db::submission::ActiveModel {
        id: Unchanged(submission.id),
        status: Set(status.clone()),
        ..Default::default()
    })
    .await?;

    if submission.game_id.is_some() && status == Status::Correct {
        cds_queue::publish(
            "calculator",
            game_calculator::Payload {
                game_id: submission.game_id,
            },
        )
        .await?;
    }

    Ok(())
}

async fn handle_cheat(submission: &Submission, peer_team_id: i64) -> Result<Status, anyhow::Error> {
    let (Some(game_id), Some(team_id)) = (submission.game_id, submission.team_id) else {
        return Ok(Status::Incorrect);
    };

    if let (Some(team), Some(peer_team)) = (
        cds_db::team::find_by_id::<cds_db::team::Model>(team_id, game_id).await?,
        cds_db::team::find_by_id::<cds_db::team::Model>(peer_team_id, game_id).await?,
    ) {
        for t in &[team, peer_team] {
            let _ = cds_db::team::update::<Team>(cds_db::team::ActiveModel {
                id: Unchanged(t.id),
                state: Set(State::Banned),
                ..t.clone().into_active_model()
            })
            .await?;
        }
    }

    Ok(Status::Cheat)
}

async fn recover() -> Result<(), anyhow::Error> {
    let (unchecked_submissions, _) =
        cds_db::submission::find::<Submission>(FindSubmissionsOptions {
            status: Some(Status::Pending),
            sorts: Some("created_at".to_owned()),
            ..Default::default()
        })
        .await?;

    for submission in unchecked_submissions {
        let id = submission.id;
        cds_queue::publish("checker", id).await?;
    }

    Ok(())
}

async fn process_messages() -> Result<(), anyhow::Error> {
    let mut messages = cds_queue::subscribe("checker", None).await?;
    while let Some(Ok(message)) = messages.next().await {
        let payload = String::from_utf8(message.payload.to_vec())?;
        let id = payload.parse::<i64>()?;

        if let Err(err) = check(id).await {
            error!("{:?}", err);
        }

        message.double_ack().await.ok();
    }

    Ok(())
}

pub async fn init() {
    tokio::spawn(async move {
        if let Err(err) = process_messages().await {
            error!("{:?}", err);
        }
    });

    recover().await.unwrap();
    info!("Submission checker initialized successfully.");
}

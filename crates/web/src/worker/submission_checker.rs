//! Checker module is for checking submissions,
//! it will assign a status to each submission.

use std::collections::BTreeMap;

use anyhow::anyhow;
use cds_db::{entity::submission::Status, get_db};
use futures::StreamExt;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Unchanged, ColumnTrait, Condition, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use tracing::{error, info};

use crate::worker::game_calculator;

async fn check(id: i64) -> Result<(), anyhow::Error> {
    let submission = cds_db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::submission::Column::Id.eq(id))
                .add(cds_db::entity::submission::Column::Status.eq(Status::Pending)),
        )
        .one(get_db())
        .await?
        .ok_or(anyhow!("submission_not_found"))?;

    let user = if let Some(user) = cds_db::entity::user::Entity::find_by_id(submission.user_id)
        .one(get_db())
        .await?
    {
        user
    } else {
        cds_db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await?;
        return Err(anyhow!("user_not_found"));
    };

    // Get related challenge
    let challenge = if let Some(challenge) =
        cds_db::entity::challenge::Entity::find_by_id(submission.challenge_id)
            .one(get_db())
            .await?
            .map(|challenge| cds_db::transfer::Challenge::from(challenge))
    {
        challenge
    } else {
        cds_db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await?;
        return Err(anyhow!("challenge_not_found"));
    };

    let mut status: Status = Status::Incorrect;

    let operator_id = match submission.team_id {
        Some(team_id) => team_id,
        _ => submission.user_id,
    };

    match cds_checker::check(&challenge, operator_id, &submission.content).await {
        Ok(c_status) => match c_status {
            cds_checker::Status::Correct => status = Status::Correct,
            cds_checker::Status::Incorrect => status = Status::Incorrect,
            cds_checker::Status::Cheat(_peer_team_id) => status = Status::Cheat,
        },
        Err(_) => status = Status::Incorrect,
    };

    if status == Status::Correct {
        // Check whether the submission is duplicate.
        let is_already_correct = if let (Some(game_id), Some(team_id)) =
            (submission.game_id, submission.team_id)
        {
            cds_db::entity::submission::Entity::find()
                .filter(cds_db::entity::submission::Column::ChallengeId.eq(submission.challenge_id))
                .filter(cds_db::entity::submission::Column::GameId.eq(game_id))
                .filter(cds_db::entity::submission::Column::TeamId.eq(team_id))
                .filter(cds_db::entity::submission::Column::Status.eq(Status::Correct))
                .count(get_db())
                .await?
                > 0
        } else {
            cds_db::entity::submission::Entity::find()
                .filter(cds_db::entity::submission::Column::ChallengeId.eq(submission.challenge_id))
                .filter(cds_db::entity::submission::Column::UserId.eq(submission.user_id))
                .filter(cds_db::entity::submission::Column::GameId.is_null())
                .filter(cds_db::entity::submission::Column::TeamId.is_null())
                .filter(cds_db::entity::submission::Column::Status.eq(Status::Correct))
                .count(get_db())
                .await?
                > 0
        };

        if is_already_correct {
            status = Status::Duplicate;
        }

        if let (Some(game_id), Some(_team_id)) = (submission.game_id, submission.team_id) {
            let game_challenge = cds_db::entity::game_challenge::Entity::find()
                .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
                .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge.id))
                .one(get_db())
                .await?
                .map(|game_challenge| cds_db::transfer::GameChallenge::from(game_challenge))
                .ok_or(anyhow!("game_challenge_not_found"))?;

            let now = chrono::Utc::now().timestamp();
            if let Some(frozen_at) = game_challenge.frozen_at {
                if now > frozen_at {
                    status = Status::Invalid;
                }
            }
        }
    }

    info!(
        "Submission #{}, status: {:?}, user: {}",
        submission.id, status, user.username
    );

    let submission = cds_db::entity::submission::ActiveModel {
        id: Unchanged(submission.id),
        status: Set(status.clone()),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    if submission.game_id.is_some() && status == Status::Correct {
        cds_queue::publish("calculator", game_calculator::Payload {
            game_id: submission.game_id,
        })
        .await?;
    }

    Ok(())
}

async fn recover() {
    let unchecked_submissions = cds_db::entity::submission::Entity::find()
        .filter(cds_db::entity::submission::Column::Status.eq(Status::Pending))
        .order_by_asc(cds_db::entity::submission::Column::CreatedAt)
        .all(get_db())
        .await
        .unwrap();

    for submission in unchecked_submissions {
        let id = submission.id;
        cds_queue::publish("checker", id).await.unwrap();
    }
}

pub async fn init() {
    tokio::spawn(async move {
        let mut messages = cds_queue::subscribe("checker").await.unwrap();
        while let Some(result) = messages.next().await {
            if result.is_err() {
                continue;
            }
            let message = result.unwrap();
            let payload = String::from_utf8(message.payload.to_vec()).unwrap();
            let id = payload.parse::<i64>().unwrap();

            if let Err(err) = check(id).await {
                error!("{:?}", err);
            }

            message.ack().await.unwrap();
        }
    });
    recover().await;
    info!("Submission checker initialized successfully.");
}

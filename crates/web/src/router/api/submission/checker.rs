//! checker module is for checking submissions,
//! it will assign a status to each submission.

use std::collections::BTreeMap;

use anyhow::anyhow;
use cds_db::{entity::submission::Status, get_db};
use futures::StreamExt;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Unchanged, ColumnTrait, Condition, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};
use tracing::{error, info};

use crate::router::api::game::calculator;

async fn check(id: i64) -> Result<(), anyhow::Error> {
    let submission = cds_db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::submission::Column::Id.eq(id))
                .add(cds_db::entity::submission::Column::Status.eq(Status::Pending)),
        )
        .one(get_db())
        .await?
        .ok_or(anyhow!(""))?;

    let user = cds_db::entity::user::Entity::find_by_id(submission.user_id)
        .one(get_db())
        .await?;

    if user.is_none() {
        cds_db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await?;
        return Err(anyhow!(""));
    }

    let user = user.unwrap();

    // Get related challenge
    let challenge = cds_db::entity::challenge::Entity::find_by_id(submission.challenge_id)
        .one(get_db())
        .await?
        .map(|challenge| cds_db::transfer::Challenge::from(challenge));

    if challenge.is_none() {
        cds_db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await?;

        return Err(anyhow!(""));
    }

    let challenge = challenge.unwrap();

    let exist_submissions = cds_db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::submission::Column::ChallengeId.eq(submission.challenge_id))
                .add(submission.game_id.map_or(Condition::all(), |game_id| {
                    Condition::all().add(cds_db::entity::submission::Column::GameId.eq(game_id))
                }))
                .add(cds_db::entity::submission::Column::Status.eq(Status::Correct)),
        )
        .all(get_db())
        .await?;

    let mut status: Status = Status::Incorrect;

    let operator_id = match submission.team_id {
        Some(team_id) => team_id,
        _ => submission.user_id,
    };
    let result = cds_checker::check(challenge, operator_id, &submission.flag).await;

    if result.is_err() {
        cds_db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await?;

        return Err(anyhow!(""));
    }

    let result = result?;

    match result {
        true => status = Status::Correct,
        false => status = Status::Incorrect,
    }

    if status == Status::Correct {
        for exist_submission in exist_submissions {
            if exist_submission.user_id == submission.user_id
                || (submission.game_id.is_some() && exist_submission.team_id == submission.team_id)
            {
                status = Status::Invalid;
                break;
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
        cds_queue::publish("calculator", calculator::Payload {
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
    info!("submission checker initialized successfully.");
}

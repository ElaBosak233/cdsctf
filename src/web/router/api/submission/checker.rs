//! checker module is for checking submissions,
//! it will assign a status to each submission.

use futures::StreamExt;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set,
};
use tracing::info;

use crate::{
    db::{entity::submission::Status, get_db},
    web::router::api::game::calculator,
};

async fn check(id: i64) {
    let submission = crate::db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(crate::db::entity::submission::Column::Id.eq(id))
                .add(crate::db::entity::submission::Column::Status.eq(Status::Pending)),
        )
        .one(get_db())
        .await
        .unwrap();

    if submission.is_none() {
        return;
    }

    let submission = submission.unwrap();

    let user = crate::db::entity::user::Entity::find_by_id(submission.user_id)
        .one(get_db())
        .await
        .unwrap();

    if user.is_none() {
        crate::db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await
            .unwrap();
        return;
    }

    let user = user.unwrap();

    // Get related challenge
    let challenge = crate::db::entity::challenge::Entity::find_by_id(submission.challenge_id)
        .one(get_db())
        .await
        .unwrap();

    if challenge.is_none() {
        crate::db::entity::submission::Entity::delete_by_id(submission.id)
            .exec(get_db())
            .await
            .unwrap();
        return;
    }

    let challenge = challenge.unwrap();

    let exist_submissions = crate::db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(crate::db::entity::submission::Column::ChallengeId.eq(submission.challenge_id))
                .add(submission.game_id.map_or(Condition::all(), |game_id| {
                    Condition::all().add(crate::db::entity::submission::Column::GameId.eq(game_id))
                }))
                .add(crate::db::entity::submission::Column::Status.eq(Status::Correct)),
        )
        .all(get_db())
        .await
        .unwrap();

    let mut status: Status = Status::Incorrect;

    match challenge.is_dynamic {
        true => {
            // Dynamic challenge, verify flag correctness from pods
            let pods = crate::db::entity::pod::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            crate::db::entity::pod::Column::RemovedAt
                                .gte(chrono::Utc::now().timestamp()),
                        )
                        .add(
                            crate::db::entity::pod::Column::ChallengeId.eq(submission.challenge_id),
                        )
                        .add(submission.game_id.map_or(Condition::all(), |game_id| {
                            Condition::all().add(crate::db::entity::pod::Column::GameId.eq(game_id))
                        })),
                )
                .all(get_db())
                .await
                .unwrap();

            for pod in pods {
                if pod.flag == Some(submission.flag.clone()) {
                    if pod.user_id == submission.user_id || submission.team_id == pod.team_id {
                        status = Status::Correct;
                        break;
                    } else {
                        status = Status::Cheat;
                        break;
                    }
                }
            }
        }
        false => {
            // Static challenge
            for flag in challenge.flags.clone() {
                if flag.value == submission.flag {
                    if flag.banned {
                        status = Status::Cheat;
                        break;
                    } else {
                        status = Status::Correct;
                    }
                }
            }
        }
    }

    for exist_submission in exist_submissions {
        if exist_submission.user_id == submission.user_id
            || (submission.game_id.is_some() && exist_submission.team_id == submission.team_id)
        {
            status = Status::Invalid;
            break;
        }
    }

    info!(
        "Submission #{}, status: {:?}, user: {}",
        submission.id, status, user.username
    );

    let mut submission_active_model = submission.clone().into_active_model();
    submission_active_model.status = Set(status.clone());

    submission_active_model.update(get_db()).await.unwrap();

    if submission.game_id.is_some() && status == Status::Correct {
        crate::queue::publish(
            "calculator",
            calculator::Payload {
                game_id: submission.game_id,
            },
        )
        .await
        .unwrap();
    }
}

async fn recover() {
    let unchecked_submissions = crate::db::entity::submission::Entity::find()
        .filter(crate::db::entity::submission::Column::Status.eq(Status::Pending))
        .order_by_asc(crate::db::entity::submission::Column::CreatedAt)
        .all(get_db())
        .await
        .unwrap();

    for submission in unchecked_submissions {
        let id = submission.id;
        crate::queue::publish("checker", id).await.unwrap();
    }
}

pub async fn init() {
    tokio::spawn(async move {
        let mut messages = crate::queue::subscribe("checker").await.unwrap();
        while let Some(result) = messages.next().await {
            if result.is_err() {
                continue;
            }
            let message = result.unwrap();
            let payload = String::from_utf8(message.payload.to_vec()).unwrap();
            let id = payload.parse::<i64>().unwrap();
            check(id).await;
            message.ack().await.unwrap();
        }
    });
    recover().await;
    info!("submission checker initialized successfully.");
}

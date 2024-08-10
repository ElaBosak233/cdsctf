//! calculator module is used to calculate the pts and rank of submissions, game_teams and game_challenges

use std::collections::HashMap;

use futures::StreamExt;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{database::get_db, model::submission::Status};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub game_id: Option<i64>,
}

pub async fn calculate(game_id: i64) {
    let submissions = crate::model::submission::Entity::find()
        .filter(
            Condition::all()
                .add(crate::model::submission::Column::GameId.eq(game_id))
                .add(crate::model::submission::Column::Status.eq(Status::Correct)),
        )
        .all(&get_db())
        .await
        .unwrap();

    let game_challenges = crate::model::game_challenge::Entity::find()
        .filter(Condition::all().add(crate::model::game_challenge::Column::GameId.eq(game_id)))
        .all(&get_db())
        .await
        .unwrap();

    // categorize submissions by challenge_id
    let mut submissions_by_challenge_id: HashMap<i64, Vec<crate::model::submission::Model>> =
        HashMap::new();

    for submission in submissions {
        submissions_by_challenge_id
            .entry(submission.challenge_id)
            .or_insert_with(Vec::new)
            .push(submission);
    }

    // calculate pts and rank for each submission
    for (challenge_id, mut submissions) in submissions_by_challenge_id {
        let game_challenge = game_challenges
            .iter()
            .find(|gc| gc.challenge_id == challenge_id)
            .cloned()
            .unwrap();

        // sort submissions by created_at
        submissions.sort_by_key(|s| s.created_at);

        let base_pts = crate::util::math::curve(
            game_challenge.max_pts,
            game_challenge.min_pts,
            game_challenge.difficulty,
            submissions.len() as i64,
        );

        for (rank, submission) in submissions.iter().enumerate() {
            let mut submission = submission.clone().into_active_model();
            submission.pts = Set(match rank {
                0 => base_pts * (100 + game_challenge.first_blood_reward_ratio) / 100,
                1 => base_pts * (100 + game_challenge.second_blood_reward_ratio) / 100,
                2 => base_pts * (100 + game_challenge.third_blood_reward_ratio) / 100,
                _ => base_pts,
            });
            submission.rank = Set(rank as i64 + 1);
            submission.update(&get_db()).await.unwrap();
        }

        let pts = match submissions.len() {
            0 => base_pts * (100 + game_challenge.first_blood_reward_ratio) / 100,
            1 => base_pts * (100 + game_challenge.second_blood_reward_ratio) / 100,
            2 => base_pts * (100 + game_challenge.third_blood_reward_ratio) / 100,
            _ => base_pts,
        };
        let mut game_challenge = game_challenge.into_active_model();
        game_challenge.pts = Set(pts);
        game_challenge.update(&get_db()).await.unwrap();
    }

    // calculate pts and rank for each game_team
    let submissions = crate::model::submission::Entity::find()
        .filter(
            Condition::all()
                .add(crate::model::submission::Column::GameId.eq(game_id))
                .add(crate::model::submission::Column::Status.eq(Status::Correct)),
        )
        .all(&get_db())
        .await
        .unwrap();

    let mut game_teams = crate::model::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(crate::model::game_team::Column::GameId.eq(game_id))
                .add(crate::model::game_team::Column::IsAllowed.eq(true)),
        )
        .all(&get_db())
        .await
        .unwrap();

    let pts_by_team_id = submissions
        .iter()
        .map(|s| (s.team_id.unwrap(), s.pts))
        .collect::<HashMap<i64, i64>>();

    game_teams.sort_by(|a, b| {
        pts_by_team_id
            .get(&b.team_id)
            .unwrap_or(&0)
            .cmp(&pts_by_team_id.get(&a.team_id).unwrap_or(&0))
    });

    for (rank, game_team) in game_teams.iter().enumerate() {
        let pts = *pts_by_team_id.get(&game_team.team_id).unwrap_or(&0);
        let mut game_team = game_team.clone().into_active_model();
        game_team.rank = Set(rank as i64 + 1);
        game_team.pts = Set(pts);
        game_team.update(&get_db()).await.unwrap();
    }
}

pub async fn init() {
    tokio::spawn(async move {
        let mut messages = crate::queue::subscribe("calculator").await.unwrap();
        while let Some(result) = messages.next().await {
            if result.is_err() {
                continue;
            }
            let message = result.unwrap();
            let payload = String::from_utf8(message.payload.to_vec()).unwrap();
            let calculator_payload = serde_json::from_str::<Payload>(&payload).unwrap();

            if let Some(game_id) = calculator_payload.game_id {
                calculate(game_id).await;
            } else {
                let games = crate::model::game::Entity::find()
                    .all(&get_db())
                    .await
                    .unwrap();
                for game in games {
                    calculate(game.id).await;
                }
            }

            message.ack().await.unwrap();
        }
    });
    info!("game calculator initialized successfully.");
}

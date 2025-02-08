//! calculator module is used to calculate the pts and rank of submissions,
//! game_teams and game_challenges

use std::collections::HashMap;

use cds_db::{entity::submission::Status, get_db};
use futures::StreamExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub game_id: Option<i64>,
}

pub async fn calculate(game_id: i64) {
    let submissions = cds_db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::submission::Column::GameId.eq(game_id))
                .add(cds_db::entity::submission::Column::Status.eq(Status::Correct)),
        )
        .all(get_db())
        .await
        .unwrap();

    let game_challenges = cds_db::entity::game_challenge::Entity::find()
        .filter(Condition::all().add(cds_db::entity::game_challenge::Column::GameId.eq(game_id)))
        .all(get_db())
        .await
        .unwrap();

    for game_challenge in game_challenges {
        let mut submissions = submissions
            .clone()
            .into_iter()
            .filter(|submission| submission.challenge_id == game_challenge.challenge_id)
            .collect::<Vec<_>>();

        submissions.sort_by_key(|submission| submission.created_at);

        let base_pts = crate::util::math::curve(
            game_challenge.max_pts,
            game_challenge.min_pts,
            game_challenge.difficulty,
            submissions.len() as i64,
        );

        for (rank, submission) in submissions.iter().enumerate() {
            let mut submission = submission.clone().into_active_model();
            submission.pts =
                Set(base_pts * (100 + game_challenge.bonus_ratios.get(rank).unwrap_or(&0)) / 100);
            submission.rank = Set(rank as i64 + 1);
            submission.update(get_db()).await.unwrap();
        }

        let pts = base_pts
            * (100
                + game_challenge
                    .bonus_ratios
                    .get(submissions.len())
                    .unwrap_or(&0))
            / 100;
        let mut game_challenge = game_challenge.into_active_model();
        game_challenge.pts = Set(pts);
        game_challenge.update(get_db()).await.unwrap();
    }

    // calculate pts rank for each game_team
    let submissions = cds_db::entity::submission::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::submission::Column::GameId.eq(game_id))
                .add(cds_db::entity::submission::Column::Status.eq(Status::Correct)),
        )
        .all(get_db())
        .await
        .unwrap();

    let game_teams = cds_db::entity::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_team::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_team::Column::IsAllowed.eq(true)),
        )
        .all(get_db())
        .await
        .unwrap();

    for game_team in game_teams {
        let mut submissions = submissions
            .clone()
            .into_iter()
            .filter(|submission| submission.team_id == Some(game_team.team_id))
            .collect::<Vec<_>>();

        submissions.sort_by_key(|submission| submission.created_at);
        let mut game_team = game_team.into_active_model();
        game_team.pts = Set(submissions.iter().map(|s| s.pts).sum());
        game_team.update(get_db()).await.unwrap();
    }

    // calculate rank for each game_team
    let game_teams = cds_db::entity::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_team::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_team::Column::IsAllowed.eq(true)),
        ).order_by_desc(cds_db::entity::game_team::Column::Pts)
        .all(get_db())
        .await
        .unwrap();

    for (i, game_team) in game_teams.into_iter().enumerate() {
        let mut game_team = game_team.into_active_model();
        game_team.rank = Set(i as i64 + 1);
        game_team.update(get_db()).await.unwrap();
    }
}

pub async fn init() {
    tokio::spawn(async move {
        let mut messages = cds_queue::subscribe("calculator").await.unwrap();
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
                let games = cds_db::entity::game::Entity::find()
                    .all(get_db())
                    .await
                    .unwrap();
                for game in games {
                    calculate(game.id).await;
                }
            }

            message.ack().await.unwrap();
        }
    });
    info!("Game calculator initialized successfully.");
}

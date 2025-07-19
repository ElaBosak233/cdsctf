//! calculator module is used to calculate the pts and rank of submissions,
//! teams and game_challenges

use std::collections::HashMap;

use cds_db::{
    entity::{submission::Status, team::State},
    get_db,
    sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, Order, QueryFilter,
        QueryOrder, Set,
    },
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub game_id: Option<i64>,
}

pub async fn calculate(game_id: i64) {
    let submissions = cds_db::entity::submission::Entity::find()
        .filter(cds_db::entity::submission::Column::GameId.eq(game_id))
        .filter(cds_db::entity::submission::Column::Status.eq(Status::Correct))
        .order_by(cds_db::entity::submission::Column::CreatedAt, Order::Asc)
        .all(get_db())
        .await
        .unwrap();

    let game_challenges = cds_db::entity::game_challenge::Entity::find()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
        .all(get_db())
        .await
        .unwrap();

    for game_challenge in game_challenges {
        let challenge_submissions = submissions
            .clone()
            .into_iter()
            .filter(|submission| submission.challenge_id == game_challenge.challenge_id)
            .collect::<Vec<_>>();

        let base_pts = crate::util::math::curve(
            game_challenge.max_pts,
            game_challenge.min_pts,
            game_challenge.difficulty,
            challenge_submissions.len() as i64,
        );

        for (rank, submission) in challenge_submissions.iter().enumerate() {
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

    // calculate pts rank for each team
    let submissions = cds_db::entity::submission::Entity::find()
        .filter(cds_db::entity::submission::Column::GameId.eq(game_id))
        .filter(cds_db::entity::submission::Column::Status.eq(Status::Correct))
        .order_by(cds_db::entity::submission::Column::CreatedAt, Order::Asc)
        .all(get_db())
        .await
        .unwrap();

    let mut teams = cds_db::entity::team::Entity::find()
        .filter(cds_db::entity::team::Column::GameId.eq(game_id))
        .filter(cds_db::entity::team::Column::State.eq(State::Passed))
        .all(get_db())
        .await
        .unwrap();

    let mut team_score_map: HashMap<i64, (i64, Option<i64>)> = HashMap::new();
    for submission in &submissions {
        if let Some(team_id) = submission.team_id {
            let entry = team_score_map.entry(team_id).or_insert((0, None));
            entry.0 += submission.pts;
            entry.1 = Some(submission.created_at);
        }
    }

    teams.sort_by(|a, b| {
        let (a_pts, a_time) = team_score_map.get(&a.id).unwrap_or(&(0, None));
        let (b_pts, b_time) = team_score_map.get(&b.id).unwrap_or(&(0, None));

        b_pts.cmp(a_pts).then_with(|| a_time.cmp(b_time))
    });

    for (i, team) in teams.into_iter().enumerate() {
        let mut team_model = team.clone().into_active_model();
        if let Some((pts, _)) = team_score_map.get(&team.id) {
            team_model.pts = Set(*pts);
        }
        team_model.rank = Set(i as i64 + 1);
        team_model.update(get_db()).await.unwrap();
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

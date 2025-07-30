//! calculator module is used to calculate the pts and rank of submissions,
//! teams and game_challenges

use std::collections::HashMap;

use cds_db::{
    Game, GameChallenge, Submission,
    game::FindGameOptions,
    game_challenge::FindGameChallengeOptions,
    sea_orm::{Set, Unchanged},
    submission::{FindSubmissionsOptions, Status},
    team::{FindTeamOptions, State, Team},
};
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub game_id: Option<i64>,
}

pub async fn calculate(game_id: i64) -> Result<(), anyhow::Error> {
    let (submissions, _) = cds_db::submission::find::<Submission>(FindSubmissionsOptions {
        game_id: Some(Some(game_id)),
        status: Some(Status::Correct),
        sorts: Some("created_at".to_string()),
        ..Default::default()
    })
    .await?;

    let (game_challenges, _) =
        cds_db::game_challenge::find::<GameChallenge>(FindGameChallengeOptions {
            game_id: Some(game_id),
            ..Default::default()
        })
        .await?;

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
            let mut model = cds_db::submission::ActiveModel {
                id: Unchanged(submission.id),
                ..Default::default()
            };
            model.pts =
                Set(base_pts * (100 + game_challenge.bonus_ratios.get(rank).unwrap_or(&0)) / 100);
            model.rank = Set(rank as i64 + 1);
            let _ = cds_db::submission::update::<Submission>(model).await?;
        }

        let pts = base_pts
            * (100
                + game_challenge
                    .bonus_ratios
                    .get(submissions.len())
                    .unwrap_or(&0))
            / 100;

        let _ =
            cds_db::game_challenge::update::<GameChallenge>(cds_db::game_challenge::ActiveModel {
                game_id: Unchanged(game_challenge.game_id),
                challenge_id: Unchanged(game_challenge.challenge_id),
                pts: Set(pts),
                ..Default::default()
            })
            .await?;
    }

    // calculate pts rank for each team
    let (submissions, _) = cds_db::submission::find::<Submission>(FindSubmissionsOptions {
        game_id: Some(Some(game_id)),
        status: Some(Status::Correct),
        sorts: Some("created_at".to_string()),
        ..Default::default()
    })
    .await?;

    let (mut teams, _) = cds_db::team::find::<Team>(FindTeamOptions {
        game_id: Some(game_id),
        state: Some(State::Passed),
        ..Default::default()
    })
    .await?;

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
        let mut team_model = cds_db::team::ActiveModel {
            id: Set(team.id),
            ..Default::default()
        };

        if let Some((pts, _)) = team_score_map.get(&team.id) {
            team_model.pts = Set(*pts);
        }
        team_model.rank = Set(i as i64 + 1);

        let _ = cds_db::team::update::<Team>(team_model).await?;
    }

    Ok(())
}

async fn process_messages() -> Result<(), anyhow::Error> {
    let mut messages = cds_queue::subscribe("calculator", None).await?;
    while let Some(Ok(message)) = messages.next().await {
        let payload = String::from_utf8(message.payload.to_vec())?;
        let calculator_payload = serde_json::from_str::<Payload>(&payload)?;

        if let Some(game_id) = calculator_payload.game_id {
            calculate(game_id).await?;
        } else {
            let (games, _) = cds_db::game::find::<Game>(FindGameOptions::default()).await?;
            for game in games {
                calculate(game.id).await?;
            }
        }

        message.ack().await.unwrap();
    }

    Ok(())
}

pub async fn init() {
    tokio::spawn(async move {
        if let Err(err) = process_messages().await {
            error!("{:?}", err);
        }
    });

    info!("Game calculator initialized successfully.");
}

//! calculator module is used to calculate the pts and rank of submissions,
//! teams and game_challenges

use std::{collections::HashMap, sync::Arc};

use cds_db::{
    Game, GameChallenge, Submission,
    game::FindGameOptions,
    game_challenge::FindGameChallengeOptions,
    sea_orm::{Set, Unchanged},
    submission::{FindSubmissionsOptions, Status},
    team::{FindTeamOptions, State, Team},
};
use futures_util::{StreamExt as _, future::join_all, stream};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

async fn calculate(game_id: i64) -> Result<(), anyhow::Error> {
    let submissions = async || -> Result<Vec<Submission>, anyhow::Error> {
        let (submissions, _) = cds_db::submission::find::<Submission>(FindSubmissionsOptions {
            game_id: Some(Some(game_id)),
            status: Some(Status::Correct),
            sorts: Some("created_at".to_string()),
            ..Default::default()
        })
        .await?;

        Ok(submissions)
    };

    let mut sc: HashMap<i64, Vec<Submission>> = HashMap::new(); // submissions by challenge
    for s in submissions().await? {
        sc.entry(s.challenge_id).or_default().push(s);
    }

    let sc = Arc::new(sc);

    let (game_challenges, _) =
        cds_db::game_challenge::find::<GameChallenge>(FindGameChallengeOptions {
            game_id: Some(game_id),
            ..Default::default()
        })
        .await?;

    let futures = game_challenges.into_iter().map(|game_challenge| {
        let sc = Arc::clone(&sc);
        async move {
            let challenge_submissions = sc
                .get(&game_challenge.challenge_id)
                .cloned()
                .unwrap_or_default();

            let base_pts = crate::util::math::curve(
                game_challenge.max_pts,
                game_challenge.min_pts,
                game_challenge.difficulty,
                challenge_submissions.len() as i64,
            );

            let futures = challenge_submissions
                .iter()
                .enumerate()
                .map(|(rank, submission)| {
                    let bonus = game_challenge.bonus_ratios.get(rank).cloned().unwrap_or(0);
                    let pts = base_pts * (100 + bonus) / 100;

                    async move {
                        let model = cds_db::submission::ActiveModel {
                            id: Unchanged(submission.id),
                            pts: Set(pts),
                            rank: Set(rank as i64 + 1),
                            ..Default::default()
                        };
                        let _ = cds_db::submission::update::<Submission>(model)
                            .await
                            .map_err(|e| error!("{:?}", e));
                    }
                });

            join_all(futures).await;

            let pts = base_pts
                * (100
                    + game_challenge
                        .bonus_ratios
                        .get(challenge_submissions.len())
                        .unwrap_or(&0))
                / 100;

            if pts == game_challenge.pts {
                return;
            }

            let _ = cds_db::game_challenge::update::<GameChallenge>(
                cds_db::game_challenge::ActiveModel {
                    game_id: Unchanged(game_challenge.game_id),
                    challenge_id: Unchanged(game_challenge.challenge_id),
                    pts: Set(pts),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| error!("{:?}", e));
        }
    });

    join_all(futures).await;

    let (mut teams, _) = cds_db::team::find::<Team>(FindTeamOptions {
        game_id: Some(game_id),
        state: Some(State::Passed),
        ..Default::default()
    })
    .await?;

    let mut team_score_map: HashMap<i64, (i64, Option<i64>)> = HashMap::new();
    for submission in submissions().await? {
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

    let futures = teams.iter().enumerate().map(|(rank, team)| {
        let pts = team_score_map.get(&team.id).map(|v| v.0).unwrap_or(0);

        async move {
            let team_model = cds_db::team::ActiveModel {
                id: Set(team.id),
                pts: Set(pts),
                rank: Set(rank as i64 + 1),
                ..Default::default()
            };

            let _ = cds_db::team::update::<Team>(team_model)
                .await
                .map_err(|e| error!("{:?}", e));
        }
    });

    join_all(futures).await;

    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub game_id: Option<i64>,
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

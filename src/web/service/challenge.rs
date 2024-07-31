use std::collections::HashMap;
use std::error::Error;

use sea_orm::TryIntoModel;

use crate::model::submission::Status;
use crate::util;

pub async fn find(
    req: crate::model::challenge::request::FindRequest,
) -> Result<(Vec<crate::model::challenge::Model>, u64), ()> {
    let (mut challenges, total) = crate::model::challenge::find(
        req.id,
        req.title,
        req.category_id,
        req.is_practicable,
        req.is_dynamic,
        req.page,
        req.size,
    )
    .await
    .unwrap();

    for challenge in challenges.iter_mut() {
        let is_detailed = req.is_detailed.unwrap_or(false);
        if !is_detailed {
            challenge.flags.clear();
        }
    }

    return Ok((challenges, total));
}

pub async fn status(
    req: crate::model::challenge::request::StatusRequest,
) -> Result<HashMap<i64, crate::model::challenge::response::StatusResponse>, Box<dyn Error>> {
    let mut submissions = crate::model::submission::find_by_challenge_ids(req.cids.clone())
        .await
        .unwrap();

    let mut result: HashMap<i64, crate::model::challenge::response::StatusResponse> =
        HashMap::new();

    for cid in req.cids {
        result
            .entry(cid)
            .or_insert_with(|| crate::model::challenge::response::StatusResponse {
                is_solved: false,
                solved_times: 0,
                pts: 0,
                bloods: Vec::new(),
            });
    }

    for submission in submissions.iter_mut() {
        submission.simplify();
        submission.challenge = None;

        if req.game_id.is_some() {
            submission.game = None;
        }

        if submission.status != Status::Correct {
            continue;
        }

        let status_response = result.get_mut(&submission.challenge_id).unwrap();

        if let Some(user_id) = req.user_id {
            if submission.user_id == user_id {
                status_response.is_solved = true;
            }
        }

        if let Some(team_id) = req.team_id {
            if let Some(game_id) = req.game_id {
                if submission.team_id == Some(team_id) && submission.game_id == Some(game_id) {
                    status_response.is_solved = true;
                }
            }
        }

        status_response.solved_times += 1;
        if status_response.bloods.len() < 3 {
            status_response.bloods.push(submission.clone());
            status_response
                .bloods
                .sort_by(|a, b| a.created_at.cmp(&b.created_at));
        } else {
            let last_submission = status_response.bloods.last().unwrap();
            if submission.created_at < last_submission.created_at {
                status_response.bloods.pop();
                status_response.bloods.push(submission.clone());
                status_response
                    .bloods
                    .sort_by(|a, b| a.created_at.cmp(&b.created_at));
            }
        }
    }

    if let Some(game_id) = req.game_id {
        let (game_challenges, _) = crate::model::game_challenge::find(Some(game_id), None)
            .await
            .unwrap();

        for game_challenge in game_challenges {
            let status_response = result.get_mut(&game_challenge.challenge_id).unwrap();
            status_response.pts = util::math::curve(
                game_challenge.max_pts,
                game_challenge.min_pts,
                game_challenge.difficulty,
                status_response.solved_times,
            );
            status_response.pts = match status_response.solved_times {
                0 => status_response.pts * (100 + game_challenge.first_blood_reward_ratio) / 100,
                1 => status_response.pts * (100 + game_challenge.second_blood_reward_ratio) / 100,
                2 => status_response.pts * (100 + game_challenge.third_blood_reward_ratio) / 100,
                _ => status_response.pts,
            }
        }
    }

    return Ok(result);
}

pub async fn create(
    req: crate::model::challenge::request::CreateRequest,
) -> Result<crate::model::challenge::Model, Box<dyn Error>> {
    match crate::model::challenge::create(req.into()).await {
        Ok(challenge) => return Ok(challenge.try_into_model().unwrap()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn update(
    req: crate::model::challenge::request::UpdateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::model::challenge::update(req.into()).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::challenge::delete(id).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}
use std::{collections::HashMap, error::Error};

use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::{database::get_db, model::submission::Status, util};

pub async fn find(
    req: crate::model::submission::request::FindRequest,
) -> Result<(Vec<crate::model::submission::Model>, u64), Box<dyn Error>> {
    let (mut submissions, total) = crate::model::submission::find(
        req.id,
        req.user_id,
        req.team_id,
        req.game_id,
        req.challenge_id,
        req.status,
        req.page,
        req.size,
    )
    .await
    .unwrap();

    let is_detailed = req.is_detailed.unwrap_or(false);
    for submission in submissions.iter_mut() {
        submission.simplify();
        if !is_detailed {
            submission.blur();
        }
    }

    let game_challenges = crate::model::game_challenge::Entity::find()
        .filter(
            crate::model::game_challenge::Column::GameId.is_in(
                submissions
                    .iter()
                    .map(|s| s.game_id)
                    .collect::<Vec<Option<i64>>>(),
            ),
        )
        .filter(
            crate::model::game_challenge::Column::ChallengeId.is_in(
                submissions
                    .iter()
                    .map(|s| s.challenge_id)
                    .collect::<Vec<i64>>(),
            ),
        )
        .all(&get_db().await)
        .await
        .unwrap();

    // Calculate rank, only for game submissions
    for game_id in submissions
        .iter()
        .map(|s| s.game_id)
        .collect::<Vec<Option<i64>>>()
    {
        if let Some(game_id) = game_id {
            let mut game_submissions = submissions
                .iter_mut()
                .filter(|s| s.game_id == Some(game_id))
                .collect::<Vec<&mut crate::model::submission::Model>>();

            game_submissions.sort_by(|a, b| a.created_at.cmp(&b.created_at));

            for (i, submission) in game_submissions.iter_mut().enumerate() {
                if submission.status == Status::Correct {
                    submission.rank = Some(i as i64 + 1);
                }
            }
        }
    }

    // Calculate pts
    let mut correct_submissions_count = HashMap::new();
    for submission in &submissions {
        if submission.status == Status::Correct {
            *correct_submissions_count
                .entry((submission.game_id, submission.challenge_id))
                .or_insert(0) += 1;
        }
    }

    for submission in submissions.iter_mut() {
        if let (Some(game_id), Some(rank)) = (submission.game_id, submission.rank) {
            if let Some(gc) = game_challenges
                .iter()
                .find(|gc| gc.game_id == game_id && gc.challenge_id == submission.challenge_id)
            {
                let x = *correct_submissions_count
                    .get(&(Some(game_id), submission.challenge_id))
                    .unwrap_or(&0) as i64;

                let base_points = util::math::curve(gc.max_pts, gc.min_pts, gc.difficulty, x);
                let bonus_multiplier = match rank {
                    1 => 100 + gc.first_blood_reward_ratio,
                    2 => 100 + gc.second_blood_reward_ratio,
                    3 => 100 + gc.third_blood_reward_ratio,
                    _ => 100,
                };

                submission.pts = Some(base_points * bonus_multiplier / 100);
            }
        }
    }

    return Ok((submissions, total));
}

pub async fn create(
    req: crate::model::submission::request::CreateRequest,
) -> Result<crate::model::submission::Model, Box<dyn Error>> {
    // Get related challenge
    let (challenges, _) =
        crate::model::challenge::find(req.challenge_id, None, None, None, None, None, None)
            .await
            .unwrap();

    let challenge = challenges.first().unwrap();

    // Default submission record
    let mut submission = crate::model::submission::create(req.clone().into())
        .await
        .unwrap()
        .into_active_model();

    let (exist_submissions, _) = crate::model::submission::find(
        None,
        None,
        None,
        req.game_id,
        req.challenge_id,
        Some(Status::Correct),
        None,
        None,
    )
    .await
    .unwrap();

    let mut status: Status = Status::Incorrect;

    match challenge.is_dynamic {
        true => {
            // Dynamic challenge, verify flag correctness from pods
            let (pods, _) = crate::model::pod::find(
                None,
                None,
                None,
                None,
                req.game_id,
                Some(challenge.id),
                Some(true),
            )
            .await
            .unwrap();

            for pod in pods {
                if pod.flag == Some(req.clone().flag) {
                    if Some(pod.user_id) == req.user_id || req.team_id == pod.team_id {
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
                if flag.value == req.flag {
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
        if Some(exist_submission.user_id) == req.user_id
            || (req.game_id.is_some() && exist_submission.team_id == req.team_id)
        {
            status = Status::Invalid;
            break;
        }
    }

    submission.status = Set(status.clone());

    let submission = crate::model::submission::update(submission).await.unwrap();

    return Ok(submission);
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::submission::delete(id).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

use std::error::Error;

use sea_orm::{IntoActiveModel, Set};

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
    if !is_detailed {
        for submission in submissions.iter_mut() {
            submission.flag.clear();
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

    let (exist_submissions, total) = crate::model::submission::find(
        None,
        None,
        None,
        req.game_id,
        req.challenge_id,
        Some(2),
        None,
        None,
    )
    .await
    .unwrap();

    let mut status: i64 = 1; // Wrong answer

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
                        status = 2; // Accept
                        break;
                    } else {
                        status = 3; // Cheat
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
                        status = 3; // Cheat
                        break;
                    } else {
                        status = 2; // Accept
                    }
                }
            }
        }
    }

    for exist_submission in exist_submissions {
        if Some(exist_submission.user_id) == req.user_id
            || (req.game_id.is_some() && exist_submission.team_id == req.team_id)
        {
            status = 4; // Invalid
            break;
        }
    }

    submission.status = Set(status);
    if status == 1 {
        submission.rank = Set((total + 1).try_into().unwrap());
    }

    let submission = crate::model::submission::update(submission).await.unwrap();

    return Ok(submission);
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::submission::delete(id).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

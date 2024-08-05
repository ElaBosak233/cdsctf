use std::{collections::HashMap, error::Error};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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
        if !is_detailed {
            submission.simplify();
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
        .all(&get_db())
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

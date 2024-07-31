use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};

use super::ActiveModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_enabled: Option<bool>,
}

impl Default for FindRequest {
    fn default() -> Self {
        FindRequest {
            game_id: None,
            challenge_id: None,
            team_id: None,
            is_enabled: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub game_id: i64,
    pub challenge_id: i64,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub first_blood_reward_ratio: Option<i64>,
    pub second_blood_reward_ratio: Option<i64>,
    pub third_blood_reward_ratio: Option<i64>,
}

impl From<CreateRequest> for ActiveModel {
    fn from(req: CreateRequest) -> Self {
        Self {
            game_id: Set(req.game_id),
            challenge_id: Set(req.challenge_id),
            difficulty: req.difficulty.map_or(NotSet, |v| Set(v)),
            is_enabled: req.is_enabled.map_or(NotSet, |v| Set(v)),
            max_pts: req.max_pts.map_or(NotSet, |v| Set(v)),
            min_pts: req.min_pts.map_or(NotSet, |v| Set(v)),
            first_blood_reward_ratio: req.first_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
            second_blood_reward_ratio: req.second_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
            third_blood_reward_ratio: req.third_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub first_blood_reward_ratio: Option<i64>,
    pub second_blood_reward_ratio: Option<i64>,
    pub third_blood_reward_ratio: Option<i64>,
}

impl From<UpdateRequest> for ActiveModel {
    fn from(req: UpdateRequest) -> Self {
        Self {
            game_id: req.game_id.map_or(NotSet, |v| Set(v)),
            challenge_id: req.challenge_id.map_or(NotSet, |v| Set(v)),
            difficulty: req.difficulty.map_or(NotSet, |v| Set(v)),
            is_enabled: req.is_enabled.map_or(NotSet, |v| Set(v)),
            max_pts: req.max_pts.map_or(NotSet, |v| Set(v)),
            min_pts: req.min_pts.map_or(NotSet, |v| Set(v)),
            first_blood_reward_ratio: req.first_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
            second_blood_reward_ratio: req.second_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
            third_blood_reward_ratio: req.third_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        }
    }
}

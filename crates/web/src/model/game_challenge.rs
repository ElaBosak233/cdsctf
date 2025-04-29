use cds_db::{sea_orm, sea_orm::FromQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct GameChallenge {
    pub game_id: i64,
    pub challenge_id: Uuid,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub difficulty: i64,
    pub bonus_ratios: Vec<i64>,
    pub max_pts: i64,
    pub min_pts: i64,
    pub pts: i64,
    pub is_enabled: bool,
    pub frozen_at: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct GameChallengeMini {
    pub game_id: i64,
    pub challenge_id: Uuid,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub pts: i64,
    pub frozen_at: Option<i64>,
}

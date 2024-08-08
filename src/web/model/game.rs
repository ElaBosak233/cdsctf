use serde::{Deserialize, Serialize};
use validator::Validate;

use super::Metadata;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    pub code: u16,
    pub data: Vec<crate::model::game::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub title: String,
    pub started_at: i64,
    pub ended_at: i64,

    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub code: u16,
    pub data: crate::model::game::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub frozed_at: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub code: u16,
    pub data: crate::model::game::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_enabled: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeResponse {
    pub code: u16,
    pub data: Vec<crate::model::game_challenge::Model>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChallengeRequest {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChallengeResponse {
    pub code: u16,
    pub data: crate::model::game_challenge::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChallengeRequest {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChallengeResponse {
    pub code: u16,
    pub data: crate::model::game_challenge::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteChallengeResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamResponse {
    pub code: u16,
    pub data: Vec<crate::model::game_team::Model>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub game_id: i64,
    pub team_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamResponse {
    pub code: u16,
    pub data: crate::model::game_team::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_allowed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamResponse {
    pub code: u16,
    pub data: crate::model::game_team::Model,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteTeamResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub status: Option<crate::model::submission::Status>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionResponse {
    pub code: u16,
    pub data: Vec<crate::model::submission::GameSubmissionModel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPosterMetadataResponse {
    pub code: u16,
    pub data: Metadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavePosterResponse {
    pub code: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletePosterResponse {
    pub code: u16,
}

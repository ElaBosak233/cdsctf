use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entity::submission::Status;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct SubmissionView {
    pub id: i64,
    pub content: String,
    pub status: Status,
    pub user_id: i64,
    pub user_name: String,
    pub user_avatar_hash: Option<String>,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub team_avatar_hash: Option<String>,
    pub game_id: Option<i64>,
    pub game_title: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,
    pub pts: i64,
    pub rank: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SubmissionSummary {
    pub id: i64,
    pub status: Status,
    pub user_id: i64,
    pub user_name: String,
    pub user_avatar_hash: Option<String>,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub team_avatar_hash: Option<String>,
    pub game_id: Option<i64>,
    pub game_title: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,
    pub pts: i64,
    pub rank: i64,
}

impl From<&SubmissionView> for SubmissionSummary {
    fn from(submission: &SubmissionView) -> Self {
        Self {
            id: submission.id,
            status: submission.status.clone(),
            user_id: submission.user_id,
            user_name: submission.user_name.clone(),
            user_avatar_hash: submission.user_avatar_hash.clone(),
            team_id: submission.team_id,
            team_name: submission.team_name.clone(),
            team_avatar_hash: submission.team_avatar_hash.clone(),
            game_id: submission.game_id,
            game_title: submission.game_title.clone(),
            challenge_id: submission.challenge_id,
            challenge_title: submission.challenge_title.clone(),
            challenge_category: submission.challenge_category,
            created_at: submission.created_at,
            pts: submission.pts,
            rank: submission.rank,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Status, SubmissionSummary, SubmissionView};

    #[test]
    fn desensitized_submission_keeps_contract_but_removes_content() {
        let submission = SubmissionView {
            id: 1,
            content: "secret flag".to_owned(),
            status: Status::Correct,
            user_id: 2,
            user_name: "user".to_owned(),
            user_avatar_hash: None,
            team_id: Some(3),
            team_name: Some("team".to_owned()),
            team_avatar_hash: None,
            game_id: Some(4),
            game_title: Some("game".to_owned()),
            challenge_id: 5,
            challenge_title: "challenge".to_owned(),
            challenge_category: 6,
            created_at: 1_700_000_000,
            pts: 100,
            rank: 1,
        };

        let value = serde_json::to_value(SubmissionSummary::from(&submission)).unwrap();
        assert!(value.get("content").is_none());
        assert_eq!(value["team_id"], 3);
        assert_eq!(value["game_id"], 4);
        assert!(value.get("hashed_password").is_none());
    }
}

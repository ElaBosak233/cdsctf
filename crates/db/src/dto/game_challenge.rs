use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameChallengeView {
    pub game_id: i64,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub difficulty: i64,
    pub bonus_ratios: Vec<i64>,
    pub max_pts: i64,
    pub min_pts: i64,
    pub pts: i64,
    pub enabled: bool,
    pub frozen_at: Option<i64>,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameChallengeSummary {
    pub game_id: i64,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub pts: i64,
    pub frozen_at: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::GameChallengeSummary;

    #[test]
    fn summary_json_excludes_admin_scoring_configuration() {
        let value = serde_json::to_value(GameChallengeSummary {
            game_id: 1,
            challenge_id: 2,
            challenge_title: "challenge".to_owned(),
            challenge_category: 3,
            pts: 100,
            frozen_at: Some(1_700_000_000),
        })
        .unwrap();

        assert_eq!(value["pts"], 100);
        assert!(value.get("difficulty").is_none());
        assert!(value.get("bonus_ratios").is_none());
    }
}

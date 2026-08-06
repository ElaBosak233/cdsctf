use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScoreboardTeam {
    pub id: i64,
    pub name: String,
    pub slogan: Option<String>,
    pub avatar_hash: Option<String>,
    pub pts: i64,
    pub rank: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScoreboardSubmission {
    pub id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub user_avatar_hash: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub pts: i64,
    pub created_at: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScoreboardEntry {
    pub team: ScoreboardTeam,
    pub submissions: Vec<ScoreboardSubmission>,
}

#[cfg(test)]
mod tests {
    use super::{ScoreboardEntry, ScoreboardSubmission, ScoreboardTeam};

    #[test]
    fn nested_scoreboard_json_is_public_only() {
        let value = serde_json::to_value(ScoreboardEntry {
            team: ScoreboardTeam {
                id: 1,
                name: "team".to_owned(),
                slogan: Some("hello".to_owned()),
                avatar_hash: Some("team-avatar".to_owned()),
                pts: 100,
                rank: 2,
            },
            submissions: vec![ScoreboardSubmission {
                id: 10,
                user_id: 20,
                user_name: "user".to_owned(),
                user_avatar_hash: Some("user-avatar".to_owned()),
                challenge_id: 30,
                challenge_title: "challenge".to_owned(),
                pts: 100,
                created_at: 1_700_000_000,
            }],
        })
        .unwrap();

        assert_eq!(
            value,
            serde_json::json!({
                "team": {
                    "id": 1,
                    "name": "team",
                    "slogan": "hello",
                    "avatar_hash": "team-avatar",
                    "pts": 100,
                    "rank": 2,
                },
                "submissions": [{
                    "id": 10,
                    "user_id": 20,
                    "user_name": "user",
                    "user_avatar_hash": "user-avatar",
                    "challenge_id": 30,
                    "challenge_title": "challenge",
                    "pts": 100,
                    "created_at": 1_700_000_000_i64,
                }],
            })
        );
    }
}

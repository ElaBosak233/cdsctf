use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct NoteView {
    pub id: i64,
    pub content: String,
    pub public: bool,
    pub user_id: i64,
    pub user_name: String,
    pub user_avatar_hash: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[cfg(test)]
mod tests {
    use super::NoteView;

    #[test]
    fn note_json_contains_projection_fields_only() {
        let value = serde_json::to_value(NoteView {
            id: 1,
            content: "note".to_owned(),
            public: true,
            user_id: 2,
            user_name: "user".to_owned(),
            user_avatar_hash: Some("avatar".to_owned()),
            challenge_id: 3,
            challenge_title: "challenge".to_owned(),
            challenge_category: 4,
            created_at: 1_700_000_000,
            updated_at: 1_700_000_001,
        })
        .unwrap();

        assert_eq!(value["content"], "note");
        assert_eq!(value["challenge_category"], 4);
        assert!(value.get("user").is_none());
        assert!(value.get("challenge").is_none());
    }
}

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entity::challenge::Instance;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct ChallengeDetail {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Vec<String>,
    pub has_instance: bool,
    pub has_attachment: bool,
    pub public: bool,
    pub has_writeup: bool,
    pub instance: Option<Instance>,
    pub checker: Option<String>,
    pub writeup: Option<String>,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChallengeView {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Vec<String>,
    pub has_instance: bool,
    pub has_attachment: bool,
    pub has_writeup: bool,
    pub writeup: Option<String>,
}

impl From<&ChallengeDetail> for ChallengeView {
    fn from(challenge: &ChallengeDetail) -> Self {
        Self {
            id: challenge.id,
            title: challenge.title.clone(),
            description: challenge.description.clone(),
            category: challenge.category,
            tags: challenge.tags.clone(),
            has_instance: challenge.has_instance,
            has_attachment: challenge.has_attachment,
            has_writeup: challenge.has_writeup,
            writeup: if challenge.has_writeup && challenge.public {
                challenge.writeup.clone()
            } else {
                None
            },
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct ChallengeSummary {
    pub id: i64,
    pub title: String,
    pub category: i32,
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::{ChallengeDetail, ChallengeView};
    use crate::entity::challenge::Instance;

    #[test]
    fn desensitize_removes_private_challenge_material() {
        let challenge = ChallengeDetail {
            id: 1,
            title: "challenge".to_owned(),
            description: "description".to_owned(),
            category: 2,
            tags: vec!["tag".to_owned()],
            has_instance: true,
            has_attachment: true,
            public: false,
            has_writeup: true,
            instance: Some(Instance::default()),
            checker: Some("checker".to_owned()),
            writeup: Some("writeup".to_owned()),
            deleted_at: None,
            created_at: 1,
            updated_at: 2,
        };

        let value = serde_json::to_value(ChallengeView::from(&challenge)).unwrap();
        assert!(value.get("instance").is_none());
        assert!(value.get("checker").is_none());
        assert!(value.get("public").is_none());
        assert!(value.get("deleted_at").is_none());
        assert_eq!(value["writeup"], serde_json::Value::Null);
    }
}

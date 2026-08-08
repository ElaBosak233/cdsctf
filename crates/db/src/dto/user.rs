use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entity::user::Group;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserAccountView {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub verified: Option<bool>,
    pub group: Group,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    #[schema(ignore)]
    pub hashed_password: String,
    pub avatar_hash: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserSummary {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub avatar_hash: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserProfile {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub description: Option<String>,
    pub avatar_hash: Option<String>,
    pub created_at: i64,
}

impl From<&UserAccountView> for UserProfile {
    fn from(user: &UserAccountView) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            username: user.username.clone(),
            description: user.description.clone(),
            avatar_hash: user.avatar_hash.clone(),
            created_at: user.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UserProfile;

    #[test]
    fn profile_serialization_excludes_account_and_moderation_state() {
        let profile = UserProfile {
            id: 1,
            name: "User".to_owned(),
            username: "user".to_owned(),
            description: Some("bio".to_owned()),
            avatar_hash: Some("avatar".to_owned()),
            created_at: 1_700_000_000,
        };

        assert_eq!(
            serde_json::to_value(profile).unwrap(),
            serde_json::json!({
                "id": 1,
                "name": "User",
                "username": "user",
                "description": "bio",
                "avatar_hash": "avatar",
                "created_at": 1_700_000_000_i64,
            })
        );
    }
}

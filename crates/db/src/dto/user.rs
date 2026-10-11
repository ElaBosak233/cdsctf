use sea_orm::{DeriveActiveEnum, EnumIter, FromQueryResult, prelude::StringLen};
use serde::{Deserialize, Serialize};

/// Coarse-grained capabilities understood by the API.
///
/// The Rust enum keeps permission checks type-safe while its stable string
/// values remain the wire and PostgreSQL representation.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Permission {
    #[sea_orm(string_value = "*")]
    #[serde(rename = "*")]
    Any,
    #[sea_orm(string_value = "basic")]
    #[serde(rename = "basic")]
    Basic,
    #[sea_orm(string_value = "management:*")]
    #[serde(rename = "management:*")]
    ManagementAny,
    #[sea_orm(string_value = "management:config")]
    #[serde(rename = "management:config")]
    ManagementConfig,
    #[sea_orm(string_value = "management:challenge")]
    #[serde(rename = "management:challenge")]
    ManagementChallenge,
    #[sea_orm(string_value = "management:game")]
    #[serde(rename = "management:game")]
    ManagementGame,
    #[sea_orm(string_value = "management:idp")]
    #[serde(rename = "management:idp")]
    ManagementIdp,
    #[sea_orm(string_value = "management:user")]
    #[serde(rename = "management:user")]
    ManagementUser,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserAccountView {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub verified: Option<bool>,
    pub banned: bool,
    pub permissions: Vec<Permission>,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    #[schema(ignore)]
    pub hashed_password: String,
    pub avatar_hash: Option<String>,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub updated_at: time::OffsetDateTime,
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
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
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
            created_at: time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap(),
        };

        assert_eq!(
            serde_json::to_value(profile).unwrap(),
            serde_json::json!({
                "id": 1,
                "name": "User",
                "username": "user",
                "description": "bio",
                "avatar_hash": "avatar",
                "created_at": "2023-11-14T22:13:20Z",
            })
        );
    }
}

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserIdpView {
    pub id: i64,
    pub user_id: i64,
    pub idp_id: i64,
    pub auth_key: String,
    pub data: Option<serde_json::Value>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserIdpSummary {
    pub id: i64,
    pub idp_id: i64,
    pub auth_key: String,
}

impl From<&UserIdpView> for UserIdpSummary {
    fn from(binding: &UserIdpView) -> Self {
        Self {
            id: binding.id,
            idp_id: binding.idp_id,
            auth_key: binding.auth_key.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UserIdpSummary, UserIdpView};

    #[test]
    fn binding_json_preserves_nullable_provider_data() {
        let value = serde_json::to_value(UserIdpView {
            id: 1,
            user_id: 2,
            idp_id: 3,
            auth_key: "opaque".to_owned(),
            data: None,
            created_at: 4,
            updated_at: 5,
        })
        .unwrap();

        assert_eq!(value["data"], serde_json::Value::Null);
        assert_eq!(value["id"], 1);
        assert!(value.get("script").is_none());

        let summary = serde_json::to_value(UserIdpSummary::from(&UserIdpView {
            id: 1,
            user_id: 2,
            idp_id: 3,
            auth_key: "opaque".to_owned(),
            data: Some(serde_json::json!({"token": "secret"})),
            created_at: 4,
            updated_at: 5,
        }))
        .unwrap();
        assert!(summary.get("user_id").is_none());
        assert!(summary.get("data").is_none());
        assert!(summary.get("created_at").is_none());
    }
}

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct IdpView {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
    pub registration_enabled: bool,
    pub avatar_hash: Option<String>,
    pub portal: Option<String>,
    pub script: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct IdpSummary {
    pub id: i64,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub portal: Option<String>,
}

impl From<&IdpView> for IdpSummary {
    fn from(idp: &IdpView) -> Self {
        Self {
            id: idp.id,
            name: idp.name.clone(),
            avatar_hash: idp.avatar_hash.clone(),
            portal: idp.portal.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IdpSummary, IdpView};

    #[test]
    fn public_idp_json_excludes_script_and_audit_fields() {
        let value = serde_json::to_value(IdpSummary::from(&IdpView {
            id: 1,
            name: "provider".to_owned(),
            enabled: true,
            registration_enabled: false,
            avatar_hash: None,
            portal: Some("https://example.com".to_owned()),
            script: "secret provider script".to_owned(),
            created_at: 2,
            updated_at: 3,
        }))
        .unwrap();

        assert!(value.get("script").is_none());
        assert!(value.get("enabled").is_none());
        assert!(value.get("registration_enabled").is_none());
        assert!(value.get("created_at").is_none());
        assert!(value.get("updated_at").is_none());
    }
}

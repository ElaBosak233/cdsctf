use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

/// Public PAT metadata. Secret material is intentionally excluded.
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct PersonalAccessTokenView {
    pub id: i64,
    pub name: String,
    #[serde(with = "crate::time_format")]
    pub expires_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format::option")]
    pub last_used_at: Option<time::OffsetDateTime>,
    #[serde(with = "crate::time_format::option")]
    pub revoked_at: Option<time::OffsetDateTime>,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
}

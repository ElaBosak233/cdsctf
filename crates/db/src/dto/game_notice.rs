use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameNoticeView {
    pub id: i64,
    pub game_id: i64,
    pub challenge_id: Option<i64>,
    pub challenge_title: Option<String>,
    pub challenge_category: Option<i32>,
    pub title: String,
    pub content: String,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct AdminGameNoticeView {
    pub id: i64,
    pub game_id: i64,
    pub challenge_id: Option<i64>,
    pub challenge_title: Option<String>,
    pub challenge_category: Option<i32>,
    pub title: String,
    pub content: String,
    pub created_by: Option<i64>,
    pub created_by_name: Option<String>,
    pub created_by_avatar_hash: Option<String>,
    pub updated_by: Option<i64>,
    pub updated_by_name: Option<String>,
    pub updated_by_avatar_hash: Option<String>,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub updated_at: time::OffsetDateTime,
}

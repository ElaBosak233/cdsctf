use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameNoticeView {
    pub id: i64,
    pub game_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}

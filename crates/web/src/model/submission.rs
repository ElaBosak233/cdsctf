use cds_db::{entity::submission::Status, sea_orm, sea_orm::FromQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Submission {
    pub id: i64,
    pub content: String,
    pub status: Status,
    pub user_id: i64,
    pub user_name: String,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub game_id: Option<i64>,
    pub game_title: Option<String>,
    pub challenge_id: Uuid,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,

    pub pts: i64,
    pub rank: i64,
}

impl Submission {
    pub fn desensitize(&self) -> Self {
        Self {
            content: "".to_owned(),
            ..self.to_owned()
        }
    }
}

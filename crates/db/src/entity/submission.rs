//! SeaORM `submission` entity — maps the `submission` table and its relations.

use async_trait::async_trait;
use sea_orm::{DeriveActiveEnum, EnumIter, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "submissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub content: String,
    pub status: Status,
    pub challenge_id: i64,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub created_at: i64,
    pub processing_at: Option<i64>,
    pub checked_at: Option<i64>,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
    #[sea_orm(default_value = 0)]
    pub rank: i64,
    #[sea_orm(belongs_to, from = "challenge_id", to = "id", on_delete = "Cascade")]
    pub challenge: BelongsTo<super::challenge::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: BelongsTo<super::user::Entity>,
    #[sea_orm(belongs_to, from = "team_id", to = "id", on_delete = "Cascade")]
    pub team: BelongsTo<Option<super::team::Entity>>,
    #[sea_orm(belongs_to, from = "game_id", to = "id", on_delete = "Cascade")]
    pub game: BelongsTo<Option<super::game::Entity>>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum Status {
    #[default]
    #[sea_orm(string_value = "queued")]
    Queued,
    #[sea_orm(string_value = "processing")]
    Processing,
    #[sea_orm(string_value = "correct")]
    Correct,
    #[sea_orm(string_value = "incorrect")]
    Incorrect,
    #[sea_orm(string_value = "cheat")]
    Cheat,
    #[sea_orm(string_value = "expired")]
    Expired,
    #[sea_orm(string_value = "duplicate")]
    Duplicate,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// SeaORM lifecycle hook executed before insert/update.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = time::OffsetDateTime::now_utc().unix_timestamp();

        if insert {
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Status;

    #[test]
    fn status_uses_stable_string_values() {
        assert_eq!(serde_json::to_value(Status::Queued).unwrap(), "queued");
        assert_eq!(
            serde_json::from_str::<Status>(r#""processing""#).unwrap(),
            Status::Processing
        );
    }
}

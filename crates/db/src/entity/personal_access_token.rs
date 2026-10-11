//! SeaORM `personal_access_token` entity — maps user-bound API credentials.

use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "personal_access_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    #[serde(skip_serializing)]
    pub token_id: String,
    #[serde(skip_serializing)]
    pub token_hash: String,
    #[serde(with = "crate::time_format")]
    pub expires_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format::option")]
    pub last_used_at: Option<time::OffsetDateTime>,
    #[serde(with = "crate::time_format::option")]
    pub revoked_at: Option<time::OffsetDateTime>,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: BelongsTo<super::user::Entity>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        if insert {
            self.created_at = Set(time::OffsetDateTime::now_utc());
        }
        Ok(self)
    }
}

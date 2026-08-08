//! SeaORM `game_notice` entity — maps the `game_notice` table and its
//! relations.

use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_notices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub game_id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub created_at: i64,
    #[sea_orm(belongs_to, from = "game_id", to = "id", on_delete = "Cascade")]
    pub game: BelongsTo<super::game::Entity>,
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

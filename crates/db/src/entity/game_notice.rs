use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::game;

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
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let ts = time::OffsetDateTime::now_utc().unix_timestamp();

        if insert {
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}

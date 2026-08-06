//! SeaORM `idp` entity — maps configured Lua-backed identity providers.

use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "idps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub enabled: bool,
    pub avatar_hash: Option<String>,
    pub portal: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub script: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[sea_orm(has_many)]
    pub user_idps: HasMany<super::user_idp::Entity>,
}

impl Model {
    pub fn desensitize(mut self) -> Self {
        self.script.clear();
        self
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = time::OffsetDateTime::now_utc().unix_timestamp();
        self.updated_at = Set(ts);
        if insert {
            self.created_at = Set(ts);
        }
        Ok(self)
    }
}

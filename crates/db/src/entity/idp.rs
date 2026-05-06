//! SeaORM `idp` entity — maps configured Rune-backed identity providers.

use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::user_idp;

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
}

impl Model {
    pub fn desensitize(mut self) -> Self {
        self.script.clear();
        self
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserIdp,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserIdp => Entity::has_many(user_idp::Entity).into(),
        }
    }
}

impl Related<user_idp::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserIdp.def()
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

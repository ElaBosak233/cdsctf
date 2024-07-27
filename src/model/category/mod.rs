pub mod request;

use axum::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::challenge;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub color: String,
    pub icon: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Challenge,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Challenge => Entity::has_many(challenge::Entity).into(),
        }
    }
}

impl Related<challenge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Challenge.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

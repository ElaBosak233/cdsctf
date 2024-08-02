pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

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

pub async fn find(
    id: Option<i64>, name: Option<String>,
) -> Result<(Vec<crate::model::category::Model>, u64), DbErr> {
    let mut query = crate::model::category::Entity::find();
    if let Some(id) = id {
        query = query.filter(crate::model::category::Column::Id.eq(id));
    }
    if let Some(name) = name {
        query = query.filter(crate::model::category::Column::Name.eq(name));
    }
    let total = query.clone().count(&get_db()).await?;
    let categories = query.all(&get_db()).await?;
    Ok((categories, total))
}

pub async fn create(
    category: crate::model::category::ActiveModel,
) -> Result<crate::model::category::Model, DbErr> {
    category.insert(&get_db()).await?.try_into_model()
}

pub async fn update(
    category: crate::model::category::ActiveModel,
) -> Result<crate::model::category::Model, DbErr> {
    category.update(&get_db()).await?.try_into_model()
}

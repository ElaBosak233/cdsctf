use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{auth, cluster, site};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "configs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "JsonBinary")]
    pub auth: auth::Config,
    #[sea_orm(column_type = "JsonBinary")]
    pub cluster: cluster::Config,
    #[sea_orm(column_type = "JsonBinary")]
    pub site: site::Config,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C>(model: Model, _db: &C, _insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait, {
        let _ = crate::cache::set("config", crate::config::Config::from(model.clone())).await;
        Ok(model)
    }
}

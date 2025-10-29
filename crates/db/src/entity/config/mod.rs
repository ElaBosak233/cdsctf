pub mod auth;
pub mod captcha;
pub mod email;
pub mod meta;

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "configs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "JsonBinary")]
    pub meta: meta::Config,
    #[sea_orm(column_type = "JsonBinary")]
    pub auth: auth::Config,
    #[sea_orm(column_type = "JsonBinary")]
    pub email: email::Config,
    #[sea_orm(column_type = "JsonBinary")]
    pub captcha: captcha::Config,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

impl Model {
    pub fn desensitize(&self) -> Self {
        Self {
            id: self.id,
            meta: self.meta.clone(),
            auth: self.auth.clone(),
            email: self.email.desensitize(),
            captcha: self.captcha.desensitize(),
        }
    }
}

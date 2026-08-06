//! SeaORM `challenge` entity — maps the `challenge` table and its relations.

use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, EntityTrait, FromJsonQueryResult, Set,
    entity::prelude::*,
};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub category: i32,
    pub tags: Vec<String>,
    pub has_instance: bool,
    pub has_attachment: bool,
    pub has_writeup: bool,
    pub public: bool,
    #[sea_orm(column_type = "JsonBinary")]
    pub instance: Option<Instance>,
    #[sea_orm(column_type = "Text")]
    pub checker: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub writeup: Option<String>,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    #[sea_orm(has_many)]
    pub submissions: HasMany<super::submission::Entity>,
    #[sea_orm(has_many)]
    pub notes: HasMany<super::note::Entity>,
    #[sea_orm(has_many, via = "game_challenge")]
    pub games: HasMany<super::game::Entity>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    FromJsonQueryResult,
    utoipa::ToSchema,
)]
pub struct Instance {
    pub duration: i64,
    pub internet: bool,
    pub containers: Vec<Container>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    FromJsonQueryResult,
    utoipa::ToSchema,
)]
pub struct Container {
    pub image: String,
    pub cpu_limit: i64,
    pub memory_limit: i64,
    pub ports: Vec<Port>,
    pub envs: Vec<EnvVar>,
    #[serde(default = "default_image_pull_policy")]
    pub image_pull_policy: String,
}

/// Default Kubernetes `imagePullPolicy` when unspecified.
fn default_image_pull_policy() -> String {
    "Always".to_string()
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    FromJsonQueryResult,
    utoipa::ToSchema,
)]
pub struct Port {
    pub port: i32,
    pub protocol: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    FromJsonQueryResult,
    utoipa::ToSchema,
)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// SeaORM lifecycle hook executed before insert/update.
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

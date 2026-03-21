//! SeaORM `challenge` entity — maps the `challenge` table and its relations.

use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, FromJsonQueryResult, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

use super::{game, game_challenge, note, submission};

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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submission,
    Note,
}

impl RelationTrait for Relation {
    /// Returns the [`RelationDef`] for this relation variant.
    fn def(&self) -> RelationDef {
        match self {
            Self::Submission => Entity::has_many(submission::Entity).into(),
            Self::Note => Entity::has_many(note::Entity).into(),
        }
    }
}

impl Related<submission::Entity> for Entity {
    /// Returns the [`RelationDef`] linking to the related [`Entity`].
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<note::Entity> for Entity {
    /// Returns the [`RelationDef`] linking to the related [`Entity`].
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl Related<game::Entity> for Entity {
    /// Returns the [`RelationDef`] linking to the related [`Entity`].
    fn to() -> RelationDef {
        game_challenge::Relation::Game.def()
    }

    /// Declares a `via` join path for related entities.
    fn via() -> Option<RelationDef> {
        Some(game_challenge::Relation::Challenge.def().rev())
    }
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

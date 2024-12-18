use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DbErr, DeriveActiveEnum,
    DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, FromJsonQueryResult,
    PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{game, game_challenge, pod, submission};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub category: i32,
    pub tags: Vec<String>,
    #[sea_orm(default_value = false)]
    pub is_dynamic: bool,
    #[sea_orm(default_value = false)]
    pub has_attachment: bool,
    #[sea_orm(default_value = false)]
    pub is_practicable: bool,
    pub image_name: Option<String>,
    #[sea_orm(default_value = 0)]
    pub cpu_limit: i64,
    #[sea_orm(default_value = 0)]
    pub memory_limit: i64,
    #[sea_orm(default_value = 1800)]
    pub duration: i64,
    pub ports: Vec<i32>,
    #[sea_orm(column_type = "JsonBinary")]
    pub envs: Vec<Env>,
    #[sea_orm(column_type = "JsonBinary")]
    pub flags: Vec<Flag>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Env {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Flag {
    #[serde(rename = "type")]
    pub type_: FlagType,
    pub banned: bool,
    pub env: Option<String>,
    pub value: String,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    EnumIter,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum FlagType {
    #[default]
    Static  = 0,
    Pattern = 1,
    Dynamic = 2,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submission,
    Pod,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submission => Entity::has_many(submission::Entity).into(),
            Self::Pod => Entity::has_many(pod::Entity).into(),
        }
    }
}

impl Related<submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        game_challenge::Relation::Game.def()
    }

    fn via() -> Option<RelationDef> {
        Some(game_challenge::Relation::Challenge.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

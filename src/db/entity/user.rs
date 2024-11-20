use async_trait::async_trait;
use sea_orm::{entity::prelude::*, Condition, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{pod, submission, team, user_team};
use crate::db::get_db;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    pub nickname: String,
    #[sea_orm(unique)]
    pub email: String,
    pub group: Group,
    pub hashed_password: String,
    #[sea_orm(default_value = false)]
    pub is_deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,
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
pub enum Group {
    #[default]
    Guest  = 0,
    Banned = 1,
    User   = 2,
    Admin  = 3,
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

impl Related<team::Entity> for Entity {
    fn to() -> RelationDef {
        user_team::Relation::Team.def()
    }

    fn via() -> Option<RelationDef> {
        Some(user_team::Relation::User.def().rev())
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
        Ok(Self {
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..self
        })
    }
}

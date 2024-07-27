pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

use super::{pod, submission, team, user_team};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    pub nickname: String,
    #[sea_orm(unique)]
    pub email: Option<String>,
    pub group: String,
    pub password: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(ignore)]
    pub teams: Vec<team::Model>,
}

impl Model {
    pub fn simplify(&mut self) {
        self.password = None;
        for team in self.teams.iter_mut() {
            team.simplify();
        }
    }
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
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

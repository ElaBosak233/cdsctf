use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::{game, game_team, pod, submission, user, user_team};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub captain_id: i64,
    pub slogan: Option<String>,
    pub invite_token: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,
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

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        user_team::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(user_team::Relation::Team.def().rev())
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        game_team::Relation::Game.def()
    }

    fn via() -> Option<RelationDef> {
        Some(game_team::Relation::Team.def().rev())
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

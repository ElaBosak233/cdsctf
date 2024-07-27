pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, Set};
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
    pub description: Option<String>,
    pub invite_token: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(ignore)]
    pub users: Vec<user::Model>,
    #[sea_orm(ignore)]
    pub captain: Option<user::Model>,
}

impl Model {
    pub fn simplify(&mut self) {
        self.invite_token = None;
        if let Some(captain) = self.captain.as_mut() {
            captain.simplify();
        }
        for user in self.users.iter_mut() {
            user.simplify();
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
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

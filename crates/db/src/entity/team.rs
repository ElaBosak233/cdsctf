use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::{game, game_team, submission, user, user_team};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub slogan: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    pub invite_token: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_locked: bool,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submission,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submission => Entity::has_many(submission::Entity).into(),
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
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = chrono::Utc::now().timestamp();

        self.updated_at = Set(ts);

        if insert {
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}

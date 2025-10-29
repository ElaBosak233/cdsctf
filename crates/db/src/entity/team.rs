use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{game, team_user, user};
use crate::entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub has_avatar: bool,
    pub has_write_up: bool,

    pub state: State,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
    #[sea_orm(default_value = 0)]
    pub rank: i64,
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
pub enum State {
    Banned = 0,
    #[default]
    Preparing = 1,
    Pending = 2,
    Passed = 3,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
    Submission,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Submission => Entity::has_many(entity::submission::Entity)
                .from(Column::Id)
                .to(entity::submission::Column::TeamId)
                .into(),
        }
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        team_user::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(team_user::Relation::Team.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::{game, team_user, user};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub slogan: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_allowed: bool,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
    #[sea_orm(default_value = 0)]
    pub rank: i64,

    pub deleted_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
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

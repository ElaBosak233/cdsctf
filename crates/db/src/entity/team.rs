use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::{entity, get_db};
use crate::traits::EagerLoading;
use crate::transfer::{Team, User};
use super::{game, team_user, user};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
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
    Banned    = 0,
    #[default]
    Preparing = 1,
    Pending   = 2,
    Passed    = 3,
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

#[async_trait]
impl EagerLoading<Vec<Team>> for Vec<Model> {
    async fn eager_load<C>(self, db: &C) -> Result<Vec<Team>, DbErr>
    where C: ConnectionTrait {
        let users = self
            .load_many_to_many(user::Entity, team_user::Entity, db)
            .await?
            .into_iter()
            .map(|users| {
                users
                    .into_iter()
                    .map(User::from)
                    .collect::<Vec<User>>()
            })
            .collect::<Vec<Vec<User>>>();

        let mut teams = self
            .clone()
            .into_iter()
            .map(Team::from)
            .collect::<Vec<Team>>();

        for (i, team) in teams.iter_mut().enumerate() {
            team.users = users[i].clone();
            for user in team.users.iter_mut() {
                user.desensitize();
            }
        }

        Ok(teams)
    }
}
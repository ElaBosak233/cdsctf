pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, FromJsonQueryResult, Set};
use serde::{Deserialize, Serialize};

use super::{challenge, game, team, user};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pods")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub flag: Option<String>, // The generated flag, which will be injected into the container.
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: i64,
    #[sea_orm(column_type = "Json")]
    pub nats: Vec<Nat>,
    pub removed_at: i64,
    pub created_at: i64,

    #[sea_orm(ignore)]
    pub user: Option<user::Model>,
    #[sea_orm(ignore)]
    pub team: Option<team::Model>,
    #[sea_orm(ignore)]
    pub challenge: Option<challenge::Model>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Default)]
pub struct Nat {
    pub src: String,
    pub dst: String,
    pub protocol: String,
    pub proxy: Option<String>,
    pub entry: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Challenge,
    User,
    Team,
    Game,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Challenge => Entity::belongs_to(challenge::Entity).from(Column::ChallengeId).to(challenge::Column::Id).into(),
            Self::User => Entity::belongs_to(user::Entity).from(Column::UserId).to(user::Column::Id).into(),
            Self::Team => Entity::belongs_to(team::Entity).from(Column::TeamId).to(team::Column::Id).into(),
            Self::Game => Entity::belongs_to(game::Entity).from(Column::GameId).to(game::Column::Id).into(),
        }
    }
}

impl Related<challenge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Challenge.def()
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().timestamp()),
            ..ActiveModelTrait::default()
        }
    }
}

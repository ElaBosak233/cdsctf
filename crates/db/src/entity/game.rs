use async_trait::async_trait;
use sea_orm::{entity::prelude::*, FromJsonQueryResult, Set};
use serde::{Deserialize, Serialize};

use super::{challenge, game_challenge, game_team, submission};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub sketch: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    pub is_enabled: bool,
    pub is_public: bool,
    #[sea_orm(default_value = 3)]
    pub member_limit_min: i64,
    #[sea_orm(default_value = 3)]
    pub member_limit_max: i64,
    #[sea_orm(default_value = false)]
    pub is_need_write_up: bool,
    #[sea_orm(column_type = "JsonBinary")]
    pub timeslots: Vec<Timeslot>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Timeslot {
    pub label: String,
    pub started_at: i64,
    pub ended_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submission,
    GameTeam,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submission => Entity::has_many(submission::Entity).into(),
            Self::GameTeam => Entity::has_many(game_team::Entity).into(),
        }
    }
}

impl Related<challenge::Entity> for Entity {
    fn to() -> RelationDef {
        game_challenge::Relation::Challenge.def()
    }

    fn via() -> Option<RelationDef> {
        Some(game_challenge::Relation::Game.def().rev())
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

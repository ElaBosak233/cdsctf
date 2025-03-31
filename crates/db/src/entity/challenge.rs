use std::collections::HashMap;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, FromJsonQueryResult, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

use super::{game, game_challenge, submission};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "challenges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: uuid::Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub category: i32,
    pub tags: Vec<String>,
    pub is_dynamic: bool,
    pub has_attachment: bool,
    pub is_public: bool,
    #[sea_orm(column_type = "JsonBinary")]
    pub env: Option<Env>,
    #[sea_orm(column_type = "Text")]
    pub checker: Option<String>,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Env {
    pub duration: i64,
    pub internet: bool,
    pub containers: Vec<Container>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Container {
    pub image: String,
    pub secret: Option<String>,
    pub cpu_limit: i64,
    pub memory_limit: i64,
    pub ports: Vec<i32>,
    pub envs: HashMap<String, String>,
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

impl Related<submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        game_challenge::Relation::Game.def()
    }

    fn via() -> Option<RelationDef> {
        Some(game_challenge::Relation::Challenge.def().rev())
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
            self.id = Set(uuid::Uuid::new_v4());
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}

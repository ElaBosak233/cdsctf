pub mod request;
pub mod response;

use axum::async_trait;
use sea_orm::{entity::prelude::*, FromJsonQueryResult, Set};
use serde::{Deserialize, Serialize};

use super::{category, game, game_challenge, pod, submission};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub category_id: i64,
    #[sea_orm(default_value = false)]
    pub is_dynamic: bool,
    #[sea_orm(default_value = false)]
    pub has_attachment: bool,
    #[sea_orm(default_value = false)]
    pub is_practicable: bool,
    pub image_name: Option<String>,
    #[sea_orm(default_value = 0)]
    pub cpu_limit: i64,
    #[sea_orm(default_value = 0)]
    pub memory_limit: i64,
    #[sea_orm(default_value = 1800)]
    pub duration: i64,
    #[sea_orm(column_type = "Json")]
    pub ports: Vec<Port>,
    #[sea_orm(column_type = "Json")]
    pub envs: Vec<Env>,
    #[sea_orm(column_type = "Json")]
    pub flags: Vec<Flag>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Env {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Port {
    pub value: i64,
    pub protocol: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Flag {
    #[serde(rename = "type")]
    pub type_: String,
    pub banned: bool,
    pub env: Option<String>,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Category,
    Submission,
    Pod,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Category => {
                return Entity::belongs_to(category::Entity)
                    .from(Column::CategoryId)
                    .to(category::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .into()
            }
            Self::Submission => return Entity::has_many(submission::Entity).into(),
            Self::Pod => return Entity::has_many(pod::Entity).into(),
        }
    }
}

impl Related<category::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Category.def();
    }
}

impl Related<submission::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Submission.def();
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        return game_challenge::Relation::Game.def();
    }

    fn via() -> Option<RelationDef> {
        return Some(game_challenge::Relation::Challenge.def().rev());
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        return Self {
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..ActiveModelTrait::default()
        };
    }

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        return Ok(self);
    }
}

pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, QuerySelect, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{challenge, game_challenge, game_team, pod, submission, team};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub is_public: bool,
    #[sea_orm(default_value = 3)]
    pub member_limit_min: i64,
    #[sea_orm(default_value = 3)]
    pub member_limit_max: i64,
    #[sea_orm(default_value = 2)]
    pub parallel_container_limit: i64,
    #[sea_orm(default_value = false)]
    pub is_need_write_up: bool,
    pub started_at: i64,
    pub ended_at: i64,
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

impl Related<team::Entity> for Entity {
    fn to() -> RelationDef {
        game_team::Relation::Team.def()
    }

    fn via() -> Option<RelationDef> {
        Some(game_team::Relation::Game.def().rev())
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

pub async fn find(
    id: Option<i64>, title: Option<String>, is_enabled: Option<bool>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<crate::model::game::Model>, u64), DbErr> {
    let mut query = crate::model::game::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::game::Column::Id.eq(id));
    }

    if let Some(title) = title {
        query = query.filter(crate::model::game::Column::Title.contains(title));
    }

    if let Some(is_enabled) = is_enabled {
        query = query.filter(crate::model::game::Column::IsEnabled.eq(is_enabled));
    }

    let total = query.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let games = query.all(&get_db()).await?;

    return Ok((games, total));
}

pub mod category;
pub mod env;
pub mod flag;

use axum::async_trait;
pub use category::Category;
pub use env::Env;
pub use flag::Flag;
use sea_orm::{entity::prelude::*, FromJsonQueryResult, QuerySelect, Set};
use serde::{Deserialize, Serialize};

use super::{game, game_challenge, pod, submission};
use crate::db::get_db;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub category: Category,
    pub tags: Vec<String>,
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
    pub ports: Vec<i32>,
    #[sea_orm(column_type = "JsonBinary")]
    pub envs: Vec<Env>,
    #[sea_orm(column_type = "JsonBinary")]
    pub flags: Vec<Flag>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Ports(pub Vec<i64>);

impl Model {
    pub fn desensitize(&mut self) {
        self.envs.clear();
        self.ports.clear();
        self.flags.clear();
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
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

pub async fn find(
    id: Option<i64>, title: Option<String>, category: Option<Category>,
    is_practicable: Option<bool>, is_dynamic: Option<bool>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<Model>, u64), DbErr> {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(title) = title {
        sql = sql.filter(Column::Title.contains(title));
    }

    if let Some(category) = category {
        sql = sql.filter(Column::Category.eq(category));
    }

    if let Some(is_practicable) = is_practicable {
        sql = sql.filter(Column::IsPracticable.eq(is_practicable));
    }

    if let Some(is_dynamic) = is_dynamic {
        sql = sql.filter(Column::IsDynamic.eq(is_dynamic));
    }

    let total = sql.clone().count(get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let challenges = sql.all(get_db()).await?;

    Ok((challenges, total))
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<Model>, DbErr> {
    let challenges = Entity::find()
        .filter(Column::Id.is_in(ids))
        .all(get_db())
        .await?;

    Ok(challenges)
}

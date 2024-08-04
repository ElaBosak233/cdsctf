pub mod env;
pub mod flag;
pub mod request;
pub mod response;

use axum::async_trait;
use sea_orm::{entity::prelude::*, FromJsonQueryResult, QuerySelect, Set};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{category, game, game_challenge, pod, submission};
pub use env::Env;
pub use flag::Flag;

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
    pub ports: Vec<i32>,
    #[sea_orm(column_type = "Json")]
    pub envs: Vec<Env>,
    #[sea_orm(column_type = "Json")]
    pub flags: Vec<Flag>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Ports(pub Vec<i64>);

impl Model {
    pub fn simplify(&mut self) {
        self.envs.clear();
        self.ports.clear();
        self.flags.clear();
    }
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

pub async fn find(
    id: Option<i64>, title: Option<String>, category_id: Option<i64>, is_practicable: Option<bool>,
    is_dynamic: Option<bool>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<crate::model::challenge::Model>, u64), DbErr> {
    let mut query = crate::model::challenge::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::challenge::Column::Id.eq(id));
    }

    if let Some(title) = title {
        query = query.filter(crate::model::challenge::Column::Title.contains(title));
    }

    if let Some(category_id) = category_id {
        query = query.filter(crate::model::challenge::Column::CategoryId.eq(category_id));
    }

    if let Some(is_practicable) = is_practicable {
        query = query.filter(crate::model::challenge::Column::IsPracticable.eq(is_practicable));
    }

    if let Some(is_dynamic) = is_dynamic {
        query = query.filter(crate::model::challenge::Column::IsDynamic.eq(is_dynamic));
    }

    let total = query.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let challenges = query.all(&get_db()).await?;

    return Ok((challenges, total));
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<crate::model::challenge::Model>, DbErr> {
    let challenges = crate::model::challenge::Entity::find()
        .filter(crate::model::challenge::Column::Id.is_in(ids))
        .all(&get_db())
        .await?;

    return Ok(challenges);
}

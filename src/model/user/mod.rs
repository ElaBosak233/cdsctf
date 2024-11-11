pub mod group;

use axum::async_trait;
use group::Group;
use sea_orm::{entity::prelude::*, Condition, QuerySelect, Set};
use serde::{Deserialize, Serialize};

use super::{pod, submission, team, user_team};
use crate::database::get_db;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    pub nickname: String,
    #[sea_orm(unique)]
    pub email: String,
    pub group: Group,
    pub password: String,
    #[sea_orm(default_value = false)]
    pub is_deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(ignore)]
    pub teams: Vec<team::Model>,
}

impl Model {
    pub fn desensitize(&mut self) {
        self.password.clear();
        for team in self.teams.iter_mut() {
            team.desensitize();
        }
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

impl Related<team::Entity> for Entity {
    fn to() -> RelationDef {
        user_team::Relation::Team.def()
    }

    fn via() -> Option<RelationDef> {
        Some(user_team::Relation::User.def().rev())
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
        Ok(Self {
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..self
        })
    }
}

async fn preload(mut users: Vec<Model>) -> Result<Vec<Model>, DbErr> {
    let teams = users
        .load_many_to_many(team::Entity, user_team::Entity, &get_db())
        .await?;

    for (i, user) in users.iter_mut().enumerate() {
        user.teams = teams[i].clone();
    }

    Ok(users)
}

pub async fn find(
    id: Option<i64>, name: Option<String>, username: Option<String>, group: Option<String>,
    email: Option<String>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<Model>, u64), DbErr> {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(Column::Username.like(&pattern))
            .add(Column::Nickname.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(username) = username {
        sql = sql.filter(Column::Username.eq(username));
    }

    if let Some(group) = group {
        sql = sql.filter(Column::Group.eq(group));
    }

    if let Some(email) = email {
        sql = sql.filter(Column::Email.eq(email));
    }

    let total = sql.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let mut users = sql.all(&get_db()).await?;

    users = preload(users).await?;

    Ok((users, total))
}

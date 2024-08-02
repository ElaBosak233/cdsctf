pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, Condition, QuerySelect, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{pod, submission, team, user_team};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    pub nickname: String,
    #[sea_orm(unique)]
    pub email: Option<String>,
    pub group: String,
    pub password: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(ignore)]
    pub teams: Vec<team::Model>,
}

impl Model {
    pub fn simplify(&mut self) {
        self.password = None;
        for team in self.teams.iter_mut() {
            team.simplify();
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
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

async fn preload(
    mut users: Vec<crate::model::user::Model>,
) -> Result<Vec<crate::model::user::Model>, DbErr> {
    let teams = users
        .load_many_to_many(
            crate::model::team::Entity,
            crate::model::user_team::Entity,
            &get_db(),
        )
        .await?;

    for (i, user) in users.iter_mut().enumerate() {
        user.teams = teams[i].clone();
    }

    return Ok(users);
}

pub async fn find(
    id: Option<i64>, name: Option<String>, username: Option<String>, group: Option<String>,
    email: Option<String>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<crate::model::user::Model>, u64), DbErr> {
    let mut query = crate::model::user::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::user::Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(crate::model::user::Column::Username.like(&pattern))
            .add(crate::model::user::Column::Nickname.like(&pattern));
        query = query.filter(condition);
    }

    if let Some(username) = username {
        query = query.filter(crate::model::user::Column::Username.eq(username));
    }

    if let Some(group) = group {
        query = query.filter(crate::model::user::Column::Group.eq(group));
    }

    if let Some(email) = email {
        query = query.filter(crate::model::user::Column::Email.eq(email));
    }

    let total = query.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let mut users = query.all(&get_db()).await?;

    users = preload(users).await?;

    Ok((users, total))
}

pub async fn create(
    user: crate::model::user::ActiveModel,
) -> Result<crate::model::user::Model, DbErr> {
    user.insert(&get_db()).await?.try_into_model()
}

pub async fn update(
    user: crate::model::user::ActiveModel,
) -> Result<crate::model::user::Model, DbErr> {
    user.update(&get_db()).await?.try_into_model()
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::user::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;
    Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!(
            "User with id {} not found",
            id
        )));
    })
}

use sea_orm::{Condition, QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::Team;
use crate::{entity, entity::user::Group, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub group: Group,
    pub hashed_password: String,
    pub is_deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub teams: Vec<Team>,
}

impl From<entity::user::Model> for User {
    fn from(model: entity::user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            nickname: model.nickname,
            email: model.email,
            group: model.group,
            hashed_password: model.hashed_password,
            is_deleted: model.is_deleted,
            created_at: model.created_at,
            updated_at: model.updated_at,
            teams: vec![],
        }
    }
}

impl From<User> for entity::user::Model {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            group: user.group,
            hashed_password: user.hashed_password,
            is_deleted: user.is_deleted,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl User {
    pub fn desensitize(&mut self) {
        self.hashed_password.clear();
        for team in self.teams.iter_mut() {
            team.desensitize();
        }
    }
}

async fn preload(mut users: Vec<User>) -> Result<Vec<User>, DbErr> {
    let models = users
        .clone()
        .into_iter()
        .map(entity::user::Model::from)
        .collect::<Vec<entity::user::Model>>();
    let teams = models
        .load_many_to_many(entity::team::Entity, entity::user_team::Entity, get_db())
        .await?
        .into_iter()
        .map(|teams| teams.into_iter().map(Team::from).collect::<Vec<Team>>())
        .collect::<Vec<Vec<Team>>>();

    for (i, user) in users.iter_mut().enumerate() {
        user.teams = teams[i].clone();
    }

    Ok(users)
}

pub async fn find(
    id: Option<i64>, name: Option<String>, username: Option<String>, group: Option<String>,
    email: Option<String>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<User>, u64), DbErr> {
    let mut sql = entity::user::Entity::find();

    if let Some(id) = id {
        sql = sql.filter(entity::user::Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(entity::user::Column::Username.like(&pattern))
            .add(entity::user::Column::Nickname.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(username) = username {
        sql = sql.filter(entity::user::Column::Username.eq(username));
    }

    if let Some(group) = group {
        sql = sql.filter(entity::user::Column::Group.eq(group));
    }

    if let Some(email) = email {
        sql = sql.filter(entity::user::Column::Email.eq(email));
    }

    let total = sql.clone().count(get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let users = sql.all(get_db()).await?;
    let mut users = users.into_iter().map(User::from).collect::<Vec<User>>();

    users = preload(users).await?;

    Ok((users, total))
}

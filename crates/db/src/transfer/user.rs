use std::str::FromStr;

use sea_orm::{Condition, Order, QueryOrder, QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::{entity, entity::user::Group, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub group: Group,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn desensitize(&mut self) {
        self.hashed_password.clear();
    }
}

impl From<entity::user::Model> for User {
    fn from(model: entity::user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            nickname: model.nickname,
            email: model.email,
            group: model.group,
            description: model.description,
            hashed_password: model.hashed_password,
            deleted_at: model.deleted_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
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
            description: user.description,
            hashed_password: user.hashed_password,
            deleted_at: user.deleted_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

async fn preload(mut users: Vec<User>) -> Result<Vec<User>, DbErr> {
    let models = users
        .clone()
        .into_iter()
        .map(|user| entity::user::Model::from(user))
        .collect::<Vec<entity::user::Model>>();
    // let teams = models
    //     .load_many_to_many(entity::team::Entity, entity::game_team_user::Entity,
    // get_db())     .await?
    //     .into_iter()
    //     .map(|teams| teams.into_iter().map(Team::from).collect::<Vec<Team>>())
    //     .collect::<Vec<Vec<Team>>>();

    Ok(users)
}

pub async fn find(
    id: Option<i64>, name: Option<String>, username: Option<String>, group: Option<Group>,
    email: Option<String>, sorts: Option<String>, page: Option<u64>, size: Option<u64>,
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

    sql = sql.filter(entity::user::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match crate::entity::user::Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let users = sql.all(get_db()).await?;
    let mut users = users.into_iter().map(User::from).collect::<Vec<User>>();

    users = preload(users).await?;

    Ok((users, total))
}

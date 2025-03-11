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

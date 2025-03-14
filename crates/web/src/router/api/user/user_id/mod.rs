use argon2::PasswordHasher;
use axum::Router;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

mod avatar;

pub fn router() -> Router {
    Router::new().nest("/avatar", avatar::router())
}

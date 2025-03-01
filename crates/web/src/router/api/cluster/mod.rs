mod env;

use axum::{Router, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};

pub async fn router() -> Router {
    Router::new().nest("/envs", env::router().await)
}

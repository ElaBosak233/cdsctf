pub mod challenge;
pub mod config;
pub mod game;
pub mod media;
pub mod pod;
pub mod proxy;
pub mod submission;
pub mod team;
pub mod user;

use axum::{http::StatusCode, response::IntoResponse, Router};
use serde_json::json;

use crate::web::traits::WebResult;

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::any(index))
        .nest("/configs", config::router())
        .nest("/media", media::router())
        .nest("/proxies", proxy::router())
        .nest("/users", user::router())
        .nest("/teams", team::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router().await)
        .nest("/pods", pod::router().await)
        .nest("/submissions", submission::router().await)
}

pub async fn index() -> impl IntoResponse {
    WebResult::<()> {
        code: StatusCode::OK.as_u16(),
        msg: Some(json!("This is the heart of CdsCTF!")),
        ..WebResult::default()
    }
}

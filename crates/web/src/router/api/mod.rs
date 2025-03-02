pub mod challenge;
pub mod config;
pub mod env;
pub mod game;
pub mod media;
pub mod submission;
pub mod user;

use axum::{Router, response::IntoResponse};
use serde_json::json;

use crate::traits::WebResponse;

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::any(index))
        .nest("/configs", config::router())
        .nest("/media", media::router())
        .nest("/users", user::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router().await)
        .nest("/envs", env::router().await)
        .nest("/submissions", submission::router().await)
}

pub async fn index() -> impl IntoResponse {
    WebResponse::<()> {
        msg: Some(json!("This is the heart of CdsCTF!")),
        ..Default::default()
    }
}

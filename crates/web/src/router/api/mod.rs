pub mod challenge;
pub mod config;
pub mod game;
pub mod media;
pub mod pod;
pub mod submission;
pub mod team;
pub mod user;

use axum::{Router, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::traits::WebResponse;

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::any(index))
        .nest("/configs", config::router())
        .nest("/media", media::router())
        .nest("/users", user::router())
        .nest("/teams", team::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router().await)
        .nest("/pods", pod::router().await)
        .nest("/submissions", submission::router().await)
}

pub async fn index() -> impl IntoResponse {
    WebResponse::<()> {
        code: StatusCode::OK.as_u16(),
        msg: Some(json!("This is the heart of CdsCTF!")),
        ..Default::default()
    }
}

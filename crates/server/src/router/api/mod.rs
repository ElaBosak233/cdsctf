pub mod admin;
pub mod challenge;
pub mod config;
pub mod instance;
pub mod game;
mod media;
mod note;
pub mod submission;
pub mod user;

use std::sync::Arc;

use axum::{Router, response::IntoResponse};
use serde_json::json;

use crate::traits::{AppState, WebResponse};

pub async fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::any(index))
        .nest("/configs", config::router())
        .nest("/users", user::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router())
        .nest("/instances", instance::router())
        .nest("/submissions", submission::router())
        .nest("/notes", note::router())
        .nest("/media", media::router())
        .nest(
            "/admin",
            admin::router().route_layer(axum::middleware::from_fn(
                crate::middleware::auth::admin_only,
            )),
        )
}

pub async fn index() -> impl IntoResponse {
    WebResponse::<()> {
        msg: Some(json!("This is the heart of CdsCTF!")),
        ..Default::default()
    }
}

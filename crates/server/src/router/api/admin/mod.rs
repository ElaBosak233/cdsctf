mod challenge;
mod config;
mod game;
mod instance;
mod submission;
mod user;

use std::sync::Arc;

use axum::Router;

use crate::traits::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/users", user::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router())
        .nest("/instances", instance::router())
        .nest("/submissions", submission::router())
        .nest("/configs", config::router())
}

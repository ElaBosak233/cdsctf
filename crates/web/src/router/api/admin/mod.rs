mod challenge;
mod config;
mod game;
mod instance;
mod submission;
mod user;

use std::sync::Arc;

use axum::Router;
use utoipa_axum::router::OpenApiRouter;

use crate::traits::AppState;

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .nest("/instances", instance::router(state.clone()))
        .nest("/submissions", submission::router(state.clone()))
        .nest("/users", user::router(state.clone()))
        .nest("/challenges", challenge::router(state.clone()))
        .nest("/games", game::router(state.clone()))
        .nest("/configs", config::router(state.clone()))
}

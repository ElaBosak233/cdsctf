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

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .nest("/instances", instance::openapi_router(state.clone()))
        .nest("/submissions", submission::openapi_router(state.clone()))
        .nest("/users", user::openapi_router(state.clone()))
        .nest("/challenges", challenge::openapi_router(state.clone()))
        .nest("/games", game::openapi_router(state.clone()))
        .nest("/configs", config::openapi_router(state.clone()))
}


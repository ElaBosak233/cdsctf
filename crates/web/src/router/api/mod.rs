pub mod admin;
pub mod challenge;
pub mod config;
pub mod game;
pub mod instance;
mod media;
mod note;
pub mod submission;
pub mod user;

use std::sync::Arc;

use axum::{Json, Router};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::AppState;

/// Full `/api` route tree (with OpenAPI metadata) for nesting under
/// [`OpenApiRouter::nest("/api", ...)`].
pub fn openapi_documented_under_api(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(index).with_state(state.clone()))
        .nest("/configs", config::router(state.clone()))
        .nest("/users", user::router(state.clone()))
        .nest("/challenges", challenge::router(state.clone()))
        .nest("/games", game::router(state.clone()))
        .nest("/instances", instance::router(state.clone()))
        .nest("/notes", note::router(state.clone()))
        .nest("/media", media::router(state.clone()))
        .nest("/submissions", submission::router(state.clone()))
        .nest(
            "/admin",
            admin::router(state.clone()).route_layer(axum::middleware::from_fn(
                crate::middleware::auth::admin_only,
            )),
        )
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct IndexResponse {
    pub message: serde_json::Value,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses((status = 200, description = "Welcome payload", body = IndexResponse))
)]
pub async fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        message: json!("This is the heart of CdsCTF!"),
    })
}

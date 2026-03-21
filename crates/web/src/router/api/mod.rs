//! Public HTTP API under `/api`: nests feature routers and attaches admin auth
//! middleware.
//!
//! [`openapi_documented_under_api`] is the **single** OpenAPI-aware tree merged
//! into the top-level [`crate::docs::ApiDoc`] in [`crate::router::router`].

/// Defines the `admin` submodule (see sibling `*.rs` files).
pub mod admin;

/// Defines the `challenge` submodule (see sibling `*.rs` files).
pub mod challenge;

/// Defines the `config` submodule (see sibling `*.rs` files).
pub mod config;

/// Defines the `game` submodule (see sibling `*.rs` files).
pub mod game;

/// Defines the `instance` submodule (see sibling `*.rs` files).
pub mod instance;

/// Defines the `media` submodule (see sibling `*.rs` files).
mod media;

/// Defines the `note` submodule (see sibling `*.rs` files).
mod note;

/// Defines the `submission` submodule (see sibling `*.rs` files).
pub mod submission;

/// Defines the `user` submodule (see sibling `*.rs` files).
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

/// Welcome payload for `GET /api/`.
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct IndexResponse {
    /// Arbitrary JSON message (marketing / version hints).
    pub message: serde_json::Value,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses((status = 200, description = "Welcome payload", body = IndexResponse))
)]

/// HTTP handler for the API index route.
pub async fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        message: json!("This is the heart of CdsCTF!"),
    })
}

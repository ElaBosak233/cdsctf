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

/// 在 [`OpenApiRouter::nest("/api", ...)`] 内使用：完整 `/api` 子树（含 OpenAPI 元数据）。
pub fn openapi_documented_under_api(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(index).with_state(state.clone()))
        .nest("/configs", config::openapi_router(state.clone()))
        .nest("/users", user::openapi_router(state.clone()))
        .nest("/challenges", challenge::openapi_router(state.clone()))
        .nest("/games", game::openapi_router(state.clone()))
        .nest("/instances", instance::openapi_router(state.clone()))
        .nest("/notes", note::openapi_router(state.clone()))
        .nest("/media", media::openapi_router(state.clone()))
        .nest("/submissions", submission::openapi_router(state.clone()))
        .nest(
            "/admin",
            admin::openapi_router(state.clone()).route_layer(axum::middleware::from_fn(
                crate::middleware::auth::admin_only,
            )),
        )
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ApiIndexResponse {
    pub message: serde_json::Value,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses((status = 200, description = "Welcome payload", body = ApiIndexResponse))
)]
pub async fn index() -> Json<ApiIndexResponse> {
    Json(ApiIndexResponse {
        message: json!("This is the heart of CdsCTF!"),
    })
}

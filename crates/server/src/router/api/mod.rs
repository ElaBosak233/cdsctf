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

use axum::{Router, response::IntoResponse};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::{AppState, WebResponse};

/// 不含 `/api/` 根与 `/api/configs` 子树（由 [`openapi_documented_under_api`] 提供），用于与文档化路由合并。
pub fn router_undocumented() -> Router<Arc<AppState>> {
    Router::new()
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

/// 在 [`OpenApiRouter::nest("/api", ...)`] 内使用：带 OpenAPI 的 `/` 与 `/configs/*`。
pub fn openapi_documented_under_api(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(index).with_state(state.clone()))
        .nest("/configs", config::openapi_router(state.clone()))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::any(index))
        .nest("/configs", config::router())
        .merge(router_undocumented())
}

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses((status = 200, description = "JSON envelope with welcome message", body = serde_json::Value))
)]
pub async fn index() -> impl IntoResponse {
    WebResponse::<()> {
        msg: Some(json!("This is the heart of CdsCTF!")),
        ..Default::default()
    }
}

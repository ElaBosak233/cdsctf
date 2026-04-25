//! HTTP routing for `container` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `container_id` submodule (see sibling `*.rs` files).
mod container_id;

use std::sync::Arc;

use axum::{Json, Router};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::{AppState, EmptyJson, WebError};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_container).with_state(state.clone()))
        .nest(
            "/{container_id}",
            OpenApiRouter::from(Router::new().with_state(state.clone())),
        )
}

/// Returns container.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-instance",
    responses(
        (status = 200, description = "OK", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_container"))]
pub async fn get_container() -> Result<Json<EmptyJson>, WebError> {
    Ok(Json(EmptyJson::default()))
}

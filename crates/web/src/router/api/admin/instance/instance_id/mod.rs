//! HTTP routing for `instance_id` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `container` submodule (see sibling `*.rs` files).
mod container;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(stop_instance).with_state(state.clone()))
        .nest("/containers", container::router(state.clone()))
}

/// Tears down Kubernetes resources for an instance.
#[utoipa::path(
    post,
    path = "/stop",
    tag = "admin-instance",
    params(
        ("instance_id" = String, Path, description = "Instance / pod identifier"),
    ),
    responses(
        (status = 200, description = "Stopped", body = EmptyJson),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "stop_instance"))]
pub async fn stop_instance(
    State(s): State<Arc<AppState>>,

    Path(instance_id): Path<String>,
) -> Result<Json<EmptyJson>, WebError> {
    let pod = s.cluster.get_pod(&instance_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/instance_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    s.cluster.delete_challenge_instance(&id).await?;

    Ok(Json(EmptyJson::default()))
}

mod container;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    traits::{AppState, EmptySuccess, WebError},
};


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(stop_instance).with_state(state.clone()))
        .nest("/containers", container::openapi_router(state.clone()))
}

#[utoipa::path(
    post,
    path = "/stop",
    tag = "admin-instance",
    params(
        ("instance_id" = String, Path, description = "Instance / pod identifier"),
    ),
    responses(
        (status = 200, description = "Stopped", body = EmptySuccess),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn stop_instance(
    State(s): State<Arc<AppState>>,

    Path(instance_id): Path<String>,
) -> Result<Json<EmptySuccess>, WebError> {
    let pod = s.cluster.get_pod(&instance_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/instance_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    s.cluster.delete_challenge_instance(&id).await?;

    Ok(Json(EmptySuccess::default()))
}

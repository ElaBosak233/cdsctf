mod container;

use std::sync::Arc;

use axum::{extract::State, Router};

use crate::{
    extract::Path,
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/stop", axum::routing::post(stop_instance))
        .nest("/containers", container::router())
}

pub async fn stop_instance(
    State(s): State<Arc<AppState>>,

    Path(instance_id): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let pod = s.cluster.get_pod(&instance_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/instance_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    s.cluster.delete_challenge_instance(&id).await?;

    Ok(WebResponse::ok())
}

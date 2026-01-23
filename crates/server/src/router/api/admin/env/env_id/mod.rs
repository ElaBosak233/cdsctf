mod container;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};

use crate::{
    extract::Path,
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/stop", axum::routing::post(stop_pod))
        .nest("/containers", container::router())
}

pub async fn stop_pod(
    State(s): State<Arc<AppState>>,

    Path(pod_id): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let pod = s.cluster.get_pod(&pod_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/env_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    s.cluster.delete_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

mod container;

use axum::{Router, http::StatusCode};

use crate::{
    extract::Path,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/renew", axum::routing::post(renew_pod))
        .route("/stop", axum::routing::post(stop_pod))
        .nest("/containers", container::router())
}

pub async fn renew_pod(Path(pod_id): Path<String>) -> Result<WebResponse<()>, WebError> {
    let pod = cds_cluster::get_pod(&pod_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/env_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    cds_cluster::renew_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn stop_pod(Path(pod_id): Path<String>) -> Result<WebResponse<()>, WebError> {
    let pod = cds_cluster::get_pod(&pod_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/env_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    cds_cluster::delete_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

use axum::{Router, extract::WebSocketUpgrade, http::StatusCode, response::IntoResponse};
use cds_db::user::Group;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;

use crate::{
    extract::{Extension, Path, Query},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/renew", axum::routing::post(renew_pod))
        .route("/stop", axum::routing::post(stop_pod))
        .route("/wsrx", axum::routing::get(wsrx))
}

pub async fn renew_pod(
    Extension(ext): Extension<AuthPrincipal>,
    Path(pod_id): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_cluster::get_pod(&pod_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/env_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let team_id = labels
        .get("cds/team_id")
        .map(|s| s.to_string())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();
    let user_id = labels
        .get("cds/user_id")
        .map(|s| s.to_string())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();

    if !(operator.id == user_id || cds_db::util::is_user_in_team(operator.id, team_id).await?) {
        return Err(WebError::Forbidden(json!("")));
    }

    // SAFETY: the creation_timestamp could be safely unwrapped.
    let started_at = pod.metadata.creation_timestamp.unwrap().0.timestamp();

    let annotations = pod.metadata.annotations.unwrap_or_default();

    let renew = annotations
        .get("cds/renew")
        .map(|s| s.to_owned())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or(3);
    let duration = annotations
        .get("cds/duration")
        .map(|s| s.to_owned())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();

    if renew == 3 {
        return Err(WebError::BadRequest(json!("no_more_renewal")));
    }

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let next_start = started_at + (renew + 1) * duration;
    if next_start - now > time::Duration::minutes(10).whole_seconds() {
        return Err(WebError::BadRequest(json!("renewal_within_10_minutes")));
    }

    cds_cluster::renew_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn stop_pod(
    Extension(ext): Extension<AuthPrincipal>,
    Path(pod_id): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_cluster::get_pod(&pod_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/env_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let team_id = labels
        .get("cds/team_id")
        .map(|s| s.to_string())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();
    let user_id = labels
        .get("cds/user_id")
        .map(|s| s.to_string())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();

    if !(operator.id == user_id || cds_db::util::is_user_in_team(operator.id, team_id).await?) {
        return Err(WebError::Forbidden(json!("")));
    }

    cds_cluster::delete_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Deserialize)]
pub struct WsrxRequest {
    pub port: u32,
}

pub async fn wsrx(
    Path(env_id): Path<String>,
    Query(query): Query<WsrxRequest>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    let port = query.port;

    Ok(ws.on_upgrade(move |socket| async move {
        let result = cds_cluster::wsrx(&env_id, port as u16, socket).await;
        if let Err(e) = result {
            debug!("Failed to link pods: {:?}", e);
        }
    }))
}

mod container;

use axum::{Router, extract::WebSocketUpgrade, http::StatusCode, response::IntoResponse};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter};
use sea_orm::ColumnTrait;
use cds_db::entity::user::Group;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;
use uuid::Uuid;
use cds_db::get_db;
use crate::{
    extract::{Extension, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/renew", axum::routing::post(renew_pod))
        .route("/stop", axum::routing::post(stop_pod))
        .route("/wsrx", axum::routing::get(wsrx))
        .nest("/containers", container::router())
}

pub async fn renew_pod(
    Extension(ext): Extension<Ext>, Path(pod_id): Path<String>,
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
        .unwrap_or_default();
    let user_id = labels
        .get("cds/user_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    let is_user_in_team = cds_db::entity::team_user::Entity::find()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team_id))
        .filter(cds_db::entity::team_user::Column::UserId.eq(user_id.clone()))
        .count(get_db()).await? > 0;

    if !(operator.group == Group::Admin
        || operator.id.to_string() == user_id
        || is_user_in_team)
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let started_at = pod.metadata.creation_timestamp.unwrap().0.timestamp();

    let annotations = pod.metadata.annotations.unwrap_or_default();

    let renew = annotations
        .get("cds/renew")
        .map(|s| s.to_owned())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap();
    let duration = annotations
        .get("cds/duration")
        .map(|s| s.to_owned())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap();

    let now = chrono::Utc::now().timestamp();

    if renew == 3 {
        return Err(WebError::BadRequest(json!("no_more_renewal")));
    }

    if now - started_at + (renew + 1) * duration > chrono::Duration::minutes(10).num_seconds() {
        return Err(WebError::BadRequest(json!("renewal_within_10_minutes")));
    }

    cds_cluster::renew_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn stop_pod(
    Extension(ext): Extension<Ext>, Path(pod_id): Path<String>,
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
        .unwrap_or_default();
    let user_id = labels
        .get("cds/user_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    let is_user_in_team = cds_db::entity::team_user::Entity::find()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team_id))
        .filter(cds_db::entity::team_user::Column::UserId.eq(user_id.clone()))
        .count(get_db()).await? > 0;

    if !(operator.group == Group::Admin
        || operator.id.to_string() == user_id
        || is_user_in_team)
    {
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
    Path(pod_id): Path<String>, Query(query): Query<WsrxRequest>, ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    let port = query.port;

    Ok(ws.on_upgrade(move |socket| async move {
        let result = cds_cluster::wsrx(&pod_id, port as u16, socket).await;
        if let Err(e) = result {
            debug!("Failed to link pods: {:?}", e);
        }
    }))
}

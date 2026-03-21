//! HTTP routing for `instance_id` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path, Query},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Paths are relative to `/instances/{instance_id}`.
pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(renew_instance).with_state(state.clone()))
        .routes(routes!(stop_instance).with_state(state.clone()))
        .routes(routes!(wsrx).with_state(state.clone()))
}

#[utoipa::path(
    post,
    path = "/renew",
    tag = "instance",
    params(
        ("instance_id" = String, Path, description = "Instance / pod identifier"),
    ),
    responses(
        (status = 200, description = "Renewed", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Extends or refreshes a player instance from the API.
pub async fn renew_instance(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(instance_id): Path<String>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = s.cluster.get_pod(&instance_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/instance_id")
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

    if !(operator.id == user_id
        || cds_db::util::is_user_in_team(&s.db.conn, operator.id, team_id).await?)
    {
        return Err(WebError::Forbidden(json!("")));
    }

    // SAFETY: the creation_timestamp could be safely unwrapped.
    let started_at = pod.metadata.creation_timestamp.unwrap().0.as_second();

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

    s.cluster.renew_challenge_instance(&id).await?;

    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    post,
    path = "/stop",
    tag = "instance",
    params(
        ("instance_id" = String, Path, description = "Instance / pod identifier"),
    ),
    responses(
        (status = 200, description = "Stopped", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Tears down Kubernetes resources for an instance.
pub async fn stop_instance(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(instance_id): Path<String>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = s.cluster.get_pod(&instance_id).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/instance_id")
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

    if !(operator.id == user_id
        || cds_db::util::is_user_in_team(&s.db.conn, operator.id, team_id).await?)
    {
        return Err(WebError::Forbidden(json!("")));
    }

    s.cluster.delete_challenge_instance(&id).await?;

    Ok(Json(EmptyJson::default()))
}

#[derive(Deserialize, Serialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct WsrxRequest {
    pub port: u32,
}

#[utoipa::path(
    get,
    path = "/wsrx",
    tag = "instance",
    params(
        ("instance_id" = String, Path, description = "Instance / pod identifier"),
        WsrxRequest,
    ),
    responses(
        (status = 101, description = "WebSocket upgrade"),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Proxies WebSocket traffic into a pod port via `wsrx`.
pub async fn wsrx(
    State(s): State<Arc<AppState>>,

    Path(instance_id): Path<String>,
    Query(query): Query<WsrxRequest>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    let port = query.port;

    Ok(ws.on_upgrade(move |socket| async move {
        let result = s.cluster.wsrx(&instance_id, port as u16, socket).await;
        if let Err(e) = result {
            debug!("Failed to link pods: {:?}", e);
        }
    }))
}

//! Authorized instance lifecycle operations and connection forwarding.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use cds_cluster::{k8s_openapi::api::core::v1::Pod, metadata::InstanceIdentity};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::Instrument;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path, Query},
    traits::{AppState, AuthPrincipal, WebError},
    util::cluster::{Instance, StopInstanceResponse},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(renew_instance).with_state(state.clone()))
        .routes(routes!(stop_instance).with_state(state.clone()))
        .routes(routes!(wsrx).with_state(state.clone()))
}

/// Resolve authorization from validated identity, including terminal pods.
async fn authorize(
    s: &AppState,
    principal: AuthPrincipal,
    pod: &Pod,
) -> Result<InstanceIdentity, WebError> {
    let operator = principal
        .operator
        .ok_or(WebError::Unauthorized(json!("unauthorized")))?;
    let identity = InstanceIdentity::parse(&pod.metadata)?;
    if operator.id != identity.creator_user_id {
        let is_member = match identity.team_id {
            Some(team_id) => {
                cds_db::team_user::contains_user(&s.db.conn, team_id, operator.id).await?
            }
            None => false,
        };
        if !is_member {
            return Err(WebError::Forbidden(json!("forbidden")));
        }
    }
    Ok(identity)
}

async fn load_authorized(
    s: &AppState,
    principal: AuthPrincipal,
    id: &str,
) -> Result<(Pod, InstanceIdentity), WebError> {
    if principal.operator.is_none() {
        return Err(WebError::Unauthorized(json!("unauthorized")));
    }
    cds_cluster::metadata::validate_id(id)
        .map_err(|_| WebError::BadRequest(json!("invalid_instance_id")))?;
    let pod = s.cluster.get_pod(id).await?;
    let identity = authorize(s, principal, &pod).await?;
    Ok((pod, identity))
}

#[utoipa::path(
    post, path = "/renew", tag = "instance",
    params(("instance_id" = String, Path, description = "Instance identifier")),
    responses(
        (status = 200, description = "Renewed instance", body = Instance),
        (status = 400, description = "Renewal is not allowed", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 409, description = "Instance changed", body = crate::traits::ErrorResponse),
        (status = 423, description = "Game paused", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "renew_instance"))]
pub async fn renew_instance(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(instance_id): Path<String>,
) -> Result<Json<Instance>, WebError> {
    let (pod, identity) = load_authorized(&s, principal, &instance_id).await?;
    if let Some(game_id) = identity.game_id {
        let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
        crate::util::loader::ensure_game_not_paused(&game)?;
    }
    let services = s
        .cluster
        .get_services_by_label(&format!("cds/instance_id={instance_id}"))
        .await?;
    let pod = s.cluster.renew_challenge_instance(&pod).await?;
    Ok(Json(Instance::from_pod(
        &pod,
        services.first(),
        &s.env,
        cds_cluster::lifecycle::now(),
    )?))
}

#[utoipa::path(
    post, path = "/stop", tag = "instance",
    params(("instance_id" = String, Path, description = "Instance identifier")),
    responses(
        (status = 200, description = "Stop requested; poll until instance is gone", body = StopInstanceResponse),
        (status = 400, description = "Invalid instance identifier", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 409, description = "Instance changed", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "stop_instance"))]
pub async fn stop_instance(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(instance_id): Path<String>,
) -> Result<Json<StopInstanceResponse>, WebError> {
    let (pod, _) = load_authorized(&s, principal, &instance_id).await?;
    s.cluster.stop_challenge_instance(&pod).await?;
    Ok(Json(StopInstanceResponse::default()))
}

#[derive(Deserialize, Serialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct WsrxRequest {
    pub port: u32,
}

#[utoipa::path(
    get, path = "/wsrx", tag = "instance",
    params(("instance_id" = String, Path, description = "Instance identifier"), WsrxRequest),
    responses(
        (status = 101, description = "WebSocket upgrade"),
        (status = 400, description = "Invalid port", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 423, description = "Game paused", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "wsrx"))]
pub async fn wsrx(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(instance_id): Path<String>,
    Query(query): Query<WsrxRequest>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    let (pod, identity) = load_authorized(&s, principal, &instance_id).await?;
    let port = u16::try_from(query.port)
        .ok()
        .filter(|port| *port != 0)
        .ok_or(WebError::BadRequest(json!("invalid_instance_port")))?;
    if !pod
        .spec
        .as_ref()
        .into_iter()
        .flat_map(|s| &s.containers)
        .flat_map(|c| c.ports.as_deref().unwrap_or_default())
        .any(|p| {
            p.container_port == i32::from(port) && p.protocol.as_deref().unwrap_or("TCP") == "TCP"
        })
    {
        return Err(WebError::BadRequest(json!("invalid_instance_port")));
    }
    if let Some(game_id) = identity.game_id {
        let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
        crate::util::loader::ensure_game_not_paused(&game)?;
    }
    let span = tracing::info_span!("instance.tunnel", instance_id = %instance_id, port);
    Ok(ws.on_upgrade(move |socket| {
        async move {
            match s.cluster.wsrx(&instance_id, port, socket).await {
                Ok(()) => tracing::info!(outcome = "succeeded", "instance tunnel completed"),
                Err(error) => {
                    tracing::warn!(outcome = "failed", error = ?error, "instance tunnel failed")
                }
            }
        }
        .instrument(span)
    }))
}

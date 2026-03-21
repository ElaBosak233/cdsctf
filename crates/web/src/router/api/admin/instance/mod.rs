//! HTTP routing for `instance` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `instance_id` submodule (see sibling `*.rs` files).
mod instance_id;

use std::{collections::BTreeMap, sync::Arc};

use axum::{Json, Router, extract::State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Query},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util::cluster::Instance,
};

/// Paths are relative to `/admin/instances`.
pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_instance).with_state(state.clone()))
        .routes(routes!(create_debug_instance).with_state(state.clone()))
        .nest("/{instance_id}", instance_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetInstanceRequest {
    pub id: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ListInstancesResponse {
    pub instances: Vec<Instance>,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-instance",
    params(GetInstanceRequest),
    responses(
        (status = 200, description = "Matching instances", body = ListInstancesResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns instance.
pub async fn get_instance(
    State(s): State<Arc<AppState>>,

    Query(params): Query<GetInstanceRequest>,
) -> Result<Json<ListInstancesResponse>, WebError> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    if let Some(id) = params.id {
        map.insert("cds/instance_id".to_owned(), id);
    }

    if let Some(user_id) = params.user_id {
        map.insert("cds/user_id".to_owned(), format!("{}", user_id));
    }

    if let Some(team_id) = params.team_id {
        map.insert("cds/team_id".to_owned(), format!("{}", team_id));
    }

    if let Some(game_id) = params.game_id {
        map.insert("cds/game_id".to_owned(), format!("{}", game_id));
    }

    if let Some(challenge_id) = params.challenge_id {
        map.insert("cds/challenge_id".to_owned(), format!("{}", challenge_id));
    }

    let labels = map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join(",");

    let pods = s.cluster.get_pods_by_label(&labels).await?;

    let envs = pods
        .into_iter()
        .map(|pod| Instance::from(pod).with_env(&s.env))
        .collect::<Vec<Instance>>();

    Ok(Json(ListInstancesResponse { instances: envs }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateDebugInstanceRequest {
    pub challenge_id: i64,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-instance",
    request_body = CreateDebugInstanceRequest,
    responses(
        (status = 200, description = "Debug instance created", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 429, description = "Too many instances", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Creates debug instance.
pub async fn create_debug_instance(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<CreateDebugInstanceRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    let _ = challenge
        .clone()
        .instance
        .ok_or(WebError::BadRequest(json!("challenge_instance_invalid")))?;

    let existing_pods = s
        .cluster
        .get_pods_by_label(
            &BTreeMap::from([("cds/user_id", format!("{}", operator.id))])
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(","),
        )
        .await?;

    if existing_pods.len() >= 1 {
        return Err(WebError::TooManyRequests(json!("too_many_user_pods")));
    }

    s.cluster
        .create_challenge_instance(operator, None, None, challenge)
        .await?;

    Ok(Json(EmptyJson::default()))
}

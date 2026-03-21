//! HTTP routing for `instance` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `instance_id` submodule (see sibling `*.rs` files).
mod instance_id;

use std::{collections::BTreeMap, sync::Arc};

use axum::{Json, Router, extract::State};
use cds_db::{TeamUser, team_user::FindTeamUserOptions};
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

/// Nests under [`OpenApiRouter::nest("/instances", ...)`]; paths are relative
/// to `/instances`.
pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_instance).with_state(state.clone()))
        .routes(routes!(create_instance).with_state(state.clone()))
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
    pub items: Vec<Instance>,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "instance",
    params(GetInstanceRequest),
    responses(
        (status = 200, description = "Matching challenge instances", body = ListInstancesResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 429, description = "Too many instances", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns instance.
pub async fn get_instance(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<GetInstanceRequest>,
) -> Result<Json<ListInstancesResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    match (params.user_id, params.team_id, params.game_id) {
        (Some(user_id), None, None) => {
            if operator.id != user_id {
                return Err(WebError::Forbidden(json!("")));
            }
            map.insert("cds/user_id".to_owned(), format!("{}", user_id));
        }
        (_, Some(team_id), Some(game_id)) => {
            let team =
                crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;
            if team.id != team_id {
                return Err(WebError::Forbidden(json!("")));
            }
            map.insert("cds/team_id".to_owned(), format!("{}", team_id));
            map.insert("cds/game_id".to_owned(), format!("{}", game_id));
        }
        _ => {
            return Err(WebError::BadRequest(json!("")));
        }
    }

    if let Some(id) = params.id {
        map.insert("cds/instance_id".to_owned(), id);
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

    let instances = pods
        .into_iter()
        .map(|pod| Instance::from(pod).with_env(&s.env))
        .collect::<Vec<Instance>>();

    Ok(Json(ListInstancesResponse { items: instances }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateInstanceRequest {
    pub challenge_id: i64,
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "instance",
    request_body = CreateInstanceRequest,
    responses(
        (status = 200, description = "Instance created", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 429, description = "Too many instances", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Creates instance.
pub async fn create_instance(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<CreateInstanceRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    if !cds_db::util::can_user_access_challenge(&s.db.conn, operator.id, challenge.id).await? {
        return Err(WebError::NotFound(json!("challenge_not_found")));
    }

    let _ = challenge
        .clone()
        .instance
        .ok_or(WebError::BadRequest(json!("challenge_instance_invalid")))?;

    if body.game_id.is_some() != body.team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let _ = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;
        let _ =
            crate::util::loader::prepare_game_challenge(&s.db.conn, game_id, challenge.id).await?;

        if !cds_db::util::is_user_in_team(&s.db.conn, operator.id, team_id).await? {
            return Err(WebError::Forbidden(json!("team_not_found")));
        }

        let (_, member_count) = cds_db::team_user::find::<TeamUser>(
            &s.db.conn,
            FindTeamUserOptions {
                team_id: Some(team_id),
                ..Default::default()
            },
        )
        .await?;

        let existing_pods = s
            .cluster
            .get_pods_by_label(
                &BTreeMap::from([
                    ("cds/game_id", format!("{}", game_id)),
                    ("cds/team_id", format!("{}", team_id)),
                ])
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(","),
            )
            .await?;

        if member_count == existing_pods.len() as u64 {
            return Err(WebError::TooManyRequests(json!("too_many_team_pods")));
        }
    } else {
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

        if existing_pods.len() == 1 {
            return Err(WebError::TooManyRequests(json!("too_many_user_pods")));
        }
    }

    let (team, game) = match (body.team_id, body.game_id) {
        (Some(team_id), Some(game_id)) => (
            cds_db::team::find_by_id(&s.db.conn, team_id, game_id).await?,
            cds_db::game::find_by_id(&s.db.conn, game_id).await?,
        ),
        _ => (None, None),
    };

    s.cluster
        .create_challenge_instance(operator, team, game, challenge)
        .await?;

    Ok(Json(EmptyJson::default()))
}

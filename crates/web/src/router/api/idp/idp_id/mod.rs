//! Public routes for a single IdP.

/// Defines the `avatar` submodule (see sibling `*.rs` files).
mod avatar;

use std::sync::Arc;

use axum::{Json, Router, extract::{Path, State}, http::StatusCode};
use cds_db::UserIdp;
use serde::Serialize;
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson},
    router::api::idp::{IdpAuthRequest, create_user_idp, enabled_idp, user_map},
    traits::{AppState, AuthPrincipal, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(bind).with_state(state.clone()))
        .nest(
            "/avatar",
            OpenApiRouter::from(Router::new())
                .routes(routes!(avatar::get_idp_avatar).with_state(state.clone())),
        )
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpBindResponse {
    pub idp: UserIdp,
}

#[utoipa::path(
    post,
    path = "/bind",
    tag = "idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    request_body = IdpAuthRequest,
    responses(
        (status = 201, description = "IdP bound", body = IdpBindResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 409, description = "Already bound", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "idp_bind"))]
pub async fn bind(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(idp_id): Path<i64>,
    ReqJson(body): ReqJson<IdpAuthRequest>,
) -> Result<(StatusCode, Json<IdpBindResponse>), WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let idp = enabled_idp(&s, idp_id).await?;
    cds_idp::Idp::preload(idp.id, &idp.script)
        .await
        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?;

    let payload = cds_idp::Idp::bind(idp.id, body.params, user_map(&s, &operator).await?).await?;

    if cds_db::user_idp::find_user_idp_by_auth_key::<UserIdp>(&s.db.conn, idp.id, &payload.auth_key)
        .await?
        .is_some()
    {
        return Err(WebError::Conflict(json!("user_idp_already_bound")));
    }
    if cds_db::user_idp::find_user_idp_by_user_and_idp::<UserIdp>(&s.db.conn, operator.id, idp.id)
        .await?
        .is_some()
    {
        return Err(WebError::Conflict(json!("idp_already_bound")));
    }

    let identity = create_user_idp(&s, &idp, operator.id, &payload).await?;
    Ok((StatusCode::CREATED, Json(IdpBindResponse { idp: identity })))
}


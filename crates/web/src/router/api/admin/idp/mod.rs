//! Admin IdP management.

/// Defines the `avatar` submodule (see sibling `*.rs` files).
mod avatar;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
};
use cds_db::{
    Idp,
    sea_orm::ActiveValue::{NotSet, Set, Unchanged},
};
use cds_engine::traits::EngineError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::Json as ReqJson,
    traits::{AppState, EmptyJson, WebError},
};

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct IdpLintResponse {
    pub markers: Vec<cds_engine::traits::DiagnosticMarker>,
}

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_idps).with_state(state.clone()))
        .routes(routes!(create_idp).with_state(state.clone()))
        .routes(routes!(get_idp).with_state(state.clone()))
        .routes(routes!(update_idp).with_state(state.clone()))
        .routes(routes!(delete_idp).with_state(state.clone()))
        .routes(routes!(lint_idp_script).with_state(state.clone()))
        .nest("/{idp_id}/avatar", avatar::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminIdpsResponse {
    pub idps: Vec<Idp>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminIdpResponse {
    pub idp: Idp,
}

#[derive(Clone, Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct AdminIdpRequest {
    #[validate(length(min = 1, max = 127))]
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub portal: Option<String>,
    pub script: String,
}

#[derive(Clone, Debug, Deserialize, utoipa::ToSchema)]
pub struct LintIdpScriptRequest {
    pub script: String,
}

fn default_true() -> bool {
    true
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-idp",
    responses((status = 200, description = "IdPs", body = AdminIdpsResponse))
)]
#[tracing::instrument(skip_all, fields(handler = "admin_list_idps"))]
pub async fn list_idps(
    State(s): State<Arc<AppState>>,
) -> Result<Json<AdminIdpsResponse>, WebError> {
    let idps = cds_db::idp::find_idps::<Idp>(&s.db.conn).await?;
    Ok(Json(AdminIdpsResponse { idps }))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-idp",
    request_body = AdminIdpRequest,
    responses(
        (status = 201, description = "Created IdP", body = AdminIdpResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "admin_create_idp"))]
pub async fn create_idp(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<AdminIdpRequest>,
) -> Result<(StatusCode, Json<AdminIdpResponse>), WebError> {
    body.validate()
        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?;
    cds_idp::Idp::lint(&body.script)
        .await
        .map_err(|err| match err {
            EngineError::DiagnosticsError(markers) => {
                WebError::BadRequest(json!({ "markers": markers }))
            }
            _ => WebError::BadRequest(json!(err.to_string())),
        })?;

    let idp = cds_db::idp::create_idp::<Idp>(
        &s.db.conn,
        cds_db::idp::IdpActiveModel {
            id: NotSet,
            name: Set(body.name),
            enabled: Set(body.enabled),
            portal: Set(body.portal),
            script: Set(body.script),
            ..Default::default()
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(AdminIdpResponse { idp })))
}

#[utoipa::path(
    get,
    path = "/{idp_id}",
    tag = "admin-idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    responses((status = 200, description = "IdP", body = AdminIdpResponse))
)]
#[tracing::instrument(skip_all, fields(handler = "admin_get_idp"))]
pub async fn get_idp(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
) -> Result<Json<AdminIdpResponse>, WebError> {
    let idp = cds_db::idp::find_idp_by_id::<Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;
    Ok(Json(AdminIdpResponse { idp }))
}

#[utoipa::path(
    put,
    path = "/{idp_id}",
    tag = "admin-idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    request_body = AdminIdpRequest,
    responses((status = 200, description = "Updated IdP", body = AdminIdpResponse))
)]
#[tracing::instrument(skip_all, fields(handler = "admin_update_idp"))]
pub async fn update_idp(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
    ReqJson(body): ReqJson<AdminIdpRequest>,
) -> Result<Json<AdminIdpResponse>, WebError> {
    body.validate()
        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?;
    let _ = cds_db::idp::find_idp_by_id::<Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;
    cds_idp::Idp::lint(&body.script)
        .await
        .map_err(|err| match err {
            EngineError::DiagnosticsError(markers) => {
                WebError::BadRequest(json!({ "markers": markers }))
            }
            _ => WebError::BadRequest(json!(err.to_string())),
        })?;

    let idp = cds_db::idp::update_idp::<Idp>(
        &s.db.conn,
        cds_db::idp::IdpActiveModel {
            id: Unchanged(idp_id),
            name: Set(body.name),
            enabled: Set(body.enabled),
            portal: Set(body.portal),
            script: Set(body.script),
            ..Default::default()
        },
    )
    .await?;
    Ok(Json(AdminIdpResponse { idp }))
}

#[utoipa::path(
    delete,
    path = "/{idp_id}",
    tag = "admin-idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    responses((status = 200, description = "Deleted", body = EmptyJson))
)]
#[tracing::instrument(skip_all, fields(handler = "admin_delete_idp"))]
pub async fn delete_idp(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    cds_db::idp::delete_idp(&s.db.conn, idp_id).await?;
    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    post,
    path = "/lint",
    tag = "admin-idp",
    request_body = LintIdpScriptRequest,
    responses(
        (status = 200, description = "Lint markers (empty if clean)", body = IdpLintResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "admin_lint_idp_script"))]
pub async fn lint_idp_script(
    ReqJson(body): ReqJson<LintIdpScriptRequest>,
) -> Result<Json<IdpLintResponse>, WebError> {
    let lint = cds_idp::Idp::lint(&body.script).await;
    let diagnostics = if let Err(lint) = lint {
        match lint {
            EngineError::DiagnosticsError(diagnostics) => Some(diagnostics),
            _ => {
                error!("{:?}", lint);
                None
            }
        }
    } else {
        None
    };

    Ok(Json(IdpLintResponse {
        markers: diagnostics.unwrap_or_default(),
    }))
}

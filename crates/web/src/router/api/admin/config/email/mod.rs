//! HTTP routing for `email` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_media::config::email::EmailType;
use serde::Deserialize;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Query},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_email).with_state(state.clone()))
        .routes(routes!(save_email).with_state(state.clone()))
}

#[derive(Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetEmailRequest {
    #[serde(rename = "type")]
    #[param(rename = "type")]
    pub type_: EmailType,
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct EmailTemplateResponse {
    pub content: String,
}

/// Returns email.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-config",
    params(GetEmailRequest),
    responses(
        (status = 200, description = "Template HTML/text", body = EmailTemplateResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_email"))]
pub async fn get_email(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetEmailRequest>,
) -> Result<Json<EmailTemplateResponse>, WebError> {
    Ok(Json(EmailTemplateResponse {
        content: s.media.config().email().get_email(params.type_).await?,
    }))
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct SaveEmailRequest {
    #[serde(rename = "type")]
    #[schema(rename = "type")]
    pub type_: EmailType,
    pub data: String,
}

/// Updates email-related media configuration.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-config",
    request_body = SaveEmailRequest,
    responses(
        (status = 200, description = "Saved", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "save_email"))]
pub async fn save_email(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<SaveEmailRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    s.media
        .config()
        .email()
        .save_email(body.type_, body.data)
        .await?;
    Ok(Json(EmptyJson::default()))
}

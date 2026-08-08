//! HTTP routing for `email` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    EmailView,
    sea_orm::{Set, Unchanged},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Json as ReqJson, Path},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_email).with_state(state.clone()))
        .routes(routes!(add_email).with_state(state.clone()))
        .routes(routes!(update_email).with_state(state.clone()))
        .routes(routes!(delete_email).with_state(state.clone()))
        .routes(routes!(verify_email).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminEmailsListResponse {
    pub emails: Vec<EmailView>,
    pub total: u64,
}

/// Returns email.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "Emails", body = AdminEmailsListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_email"))]
pub async fn get_email(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
) -> Result<Json<AdminEmailsListResponse>, WebError> {
    let emails = cds_db::email::find_by_user_id(&s.db.conn, user_id).await?;
    let total = emails.len() as u64;
    Ok(Json(AdminEmailsListResponse { emails, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct AdminAddEmailRequest {
    #[validate(email)]
    pub email: String,
    pub verified: Option<bool>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminEmailResponse {
    pub email: EmailView,
}

/// Associates a new email address with a user.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    request_body = AdminAddEmailRequest,
    responses(
        (status = 200, description = "Email added", body = AdminEmailResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "add_email"))]
pub async fn add_email(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
    ReqJson(body): ReqJson<AdminAddEmailRequest>,
) -> Result<Json<AdminEmailResponse>, WebError> {
    let email = cds_db::email::create::<EmailView>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(user_id),
            email: Set(body.email.to_lowercase()),
            verified: Set(body.verified.unwrap_or(true)),
        },
    )
    .await?;

    Ok(Json(AdminEmailResponse { email }))
}

#[derive(Clone, Debug, Deserialize, utoipa::ToSchema)]
pub struct AdminUpdateEmailRequest {
    pub verified: bool,
}

/// Updates an email address associated with a user.
#[utoipa::path(
    put,
    path = "/{mailbox}",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
        ("mailbox" = String, Path, description = "Email"),
    ),
    request_body = AdminUpdateEmailRequest,
    responses(
        (status = 200, description = "Updated", body = AdminEmailResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_email"))]
pub async fn update_email(
    State(s): State<Arc<AppState>>,
    Path((user_id, email)): Path<(i64, String)>,
    ReqJson(body): ReqJson<AdminUpdateEmailRequest>,
) -> Result<Json<AdminEmailResponse>, WebError> {
    let email =
        cds_db::email::find_by_email::<cds_db::email::Model>(&s.db.conn, email.to_lowercase())
            .await?
            .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != user_id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    let email = cds_db::email::update::<EmailView>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            email: Unchanged(email.email),
            user_id: Unchanged(email.user_id),
            verified: Set(body.verified),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(AdminEmailResponse { email }))
}

/// Deletes email.
#[utoipa::path(
    delete,
    path = "/{mailbox}",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
        ("mailbox" = String, Path, description = "Email"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_email"))]
pub async fn delete_email(
    State(s): State<Arc<AppState>>,
    Path((user_id, email)): Path<(i64, String)>,
) -> Result<Json<EmptyJson>, WebError> {
    let email = email.to_lowercase();
    let _ = cds_db::email::delete(&s.db.conn, user_id, email).await?;
    Ok(Json(EmptyJson::default()))
}

/// Confirms ownership of a pending email address.
#[utoipa::path(
    post,
    path = "/{mailbox}/verify",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
        ("mailbox" = String, Path, description = "Email"),
    ),
    responses(
        (status = 200, description = "Verified", body = AdminEmailResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "verify_email"))]
pub async fn verify_email(
    State(s): State<Arc<AppState>>,
    Path((user_id, email)): Path<(i64, String)>,
) -> Result<Json<AdminEmailResponse>, WebError> {
    let email =
        cds_db::email::find_by_email::<cds_db::email::Model>(&s.db.conn, email.to_lowercase())
            .await?
            .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != user_id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    let email = cds_db::email::update::<EmailView>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            email: Unchanged(email.email.to_owned()),
            user_id: Unchanged(email.user_id),
            verified: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(AdminEmailResponse { email }))
}

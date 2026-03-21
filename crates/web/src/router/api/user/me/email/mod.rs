//! HTTP routing for `email` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Email,
    sea_orm::{Set, Unchanged},
};
use cds_media::config::email::EmailType;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Extension, Json as ReqJson, Path},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_email).with_state(state.clone()))
        .routes(routes!(add_email).with_state(state.clone()))
        .routes(routes!(delete_email).with_state(state.clone()))
        .routes(routes!(verify_email).with_state(state.clone()))
        .routes(routes!(send_verify_email).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct EmailsListResponse {
    pub emails: Vec<Email>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    responses(
        (status = 200, description = "Linked emails", body = EmailsListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns email.
pub async fn get_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<EmailsListResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let emails = cds_db::email::find_by_user_id(&s.db.conn, operator.id).await?;
    let total = emails.len() as u64;
    Ok(Json(EmailsListResponse {
        emails,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UserAddEmailRequest {
    #[validate(email)]
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "user",
    request_body = UserAddEmailRequest,
    responses(
        (status = 200, description = "Email added", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Associates a new email address with a user.
pub async fn add_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<UserAddEmailRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let _ = cds_db::email::create::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(operator.id),
            email: Set(body.email.to_lowercase()),
            verified: Set(!cds_db::get_config(&s.db.conn).await.email.enabled),
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    delete,
    path = "/{mailbox}",
    tag = "user",
    params(
        ("mailbox" = String, Path, description = "Email address"),
    ),
    responses(
        (status = 200, description = "Removed", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Deletes email.
pub async fn delete_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let email = email.to_lowercase();

    let _ = cds_db::email::delete(&s.db.conn, operator.id, email).await?;

    Ok(Json(EmptyJson::default()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EmailVerifyRequest {
    pub code: String,
}

#[utoipa::path(
    post,
    path = "/{mailbox}/verify",
    tag = "user",
    params(
        ("mailbox" = String, Path, description = "Email address"),
    ),
    request_body = EmailVerifyRequest,
    responses(
        (status = 200, description = "Verified", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Confirms ownership of a pending email address.
pub async fn verify_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
    ReqJson(body): ReqJson<EmailVerifyRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let email = cds_db::email::find_by_email::<Email>(&s.db.conn, email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if cds_db::get_config(&s.db.conn).await.email.enabled {
        let code = s
            .cache
            .get::<String>(format!("mailbox:{}:code", email.email.to_owned()))
            .await?
            .ok_or(WebError::BadRequest("email_code_expired".into()))?;

        if code != body.code {
            return Err(WebError::BadRequest(json!("email_code_incorrect")));
        }

        let _ = s
            .cache
            .get_del::<String>(format!("mailbox:{}:code", email.email.to_owned()))
            .await?;
    }

    let _ = cds_db::email::update::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            email: Unchanged(email.email.to_owned()),
            user_id: Unchanged(email.user_id),
            verified: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    post,
    path = "/{mailbox}/verify/send",
    tag = "user",
    params(
        ("mailbox" = String, Path, description = "Email address"),
    ),
    responses(
        (status = 200, description = "Verification mail queued", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Sends an address-verification message through the mail queue.
pub async fn send_verify_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if !cds_db::get_config(&s.db.conn).await.email.enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email: Email = cds_db::email::find_by_email(&s.db.conn, email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if s.cache
        .get::<i64>(format!("mailbox:{}:buffer", email.email.to_owned()))
        .await?
        .is_some()
    {
        return Err(WebError::BadRequest(json!("email_send_too_frequently")));
    }

    let code = nanoid!();
    s.cache
        .set_ex(
            format!("mailbox:{}:code", email.email.to_owned()),
            code.to_owned(),
            60 * 60,
        )
        .await?;

    let body = s
        .media
        .config()
        .email()
        .get_email(EmailType::Verify)
        .await?;

    s.queue
        .publish(
            "mailbox",
            cds_mailbox::Payload {
                name: operator.name.to_owned(),
                email: email.email.to_owned(),
                subject: util::email::extract_title(&body)
                    .unwrap_or("Verify Your Email".to_owned()),
                body: body
                    .replace("%CODE%", &code)
                    .replace("%USER%", &operator.name),
            },
        )
        .await?;

    s.cache
        .set_ex(format!("mailbox:{}:buffer", email.email.to_owned()), 1, 60)
        .await?;

    Ok(Json(EmptyJson::default()))
}

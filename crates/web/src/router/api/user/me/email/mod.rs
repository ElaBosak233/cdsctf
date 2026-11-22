//! HTTP routing for `email` — Axum router wiring and OpenAPI route
//! registration.

use std::{sync::Arc, time::Duration};

use axum::{Json, Router, extract::State};
use cds_db::{EmailView, UserAccountView};
use cds_mailbox::{CODE_TTL, EmailPurpose, Payload};
use cds_worker::mailbox::SUBJECT;
use garde::Validate;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path, Query},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_email).with_state(state.clone()))
        .routes(routes!(add_email).with_state(state.clone()))
        .routes(routes!(send_bind_email).with_state(state.clone()))
        .routes(routes!(delete_email).with_state(state.clone()))
        .routes(routes!(verify_email).with_state(state.clone()))
        .routes(routes!(send_verify_email).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct EmailsListResponse {
    pub emails: Vec<EmailView>,
    pub total: u64,
}

/// Returns email.
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
#[tracing::instrument(skip_all, fields(handler = "get_email"))]
pub async fn get_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<EmailsListResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let emails = cds_db::email::find_by_user_id(&s.db.conn, operator.id).await?;
    let total = emails.len() as u64;
    Ok(Json(EmailsListResponse { emails, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct UserAddEmailRequest {
    #[garde(email)]
    pub email: String,
    /// Required to verify the address when email delivery is enabled.
    pub code: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AddEmailResponse {
    pub email: EmailView,
}

/// Associates a new email address with a user.
#[utoipa::path(
    post,
    path = "/",
    tag = "user",
    request_body = UserAddEmailRequest,
    responses(
        (status = 200, description = "Email added", body = AddEmailResponse),
        (status = 400, description = "Invalid email or verification code", body = crate::traits::ErrorResponse),
        (status = 409, description = "Email already linked", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "add_email"))]
pub async fn add_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<UserAddEmailRequest>,
) -> Result<Json<AddEmailResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let email_config = cds_db::get_config(&s.db.conn).await.email;
    let email = body.email.to_lowercase();
    util::email::validate_new_address(&email, &email_config.whitelist)?;

    if !cds_db::user::is_email_unique(&s.db.conn, &email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    if email_config.enabled {
        let code = body
            .code
            .as_deref()
            .map(str::trim)
            .filter(|code| !code.is_empty())
            .ok_or_else(|| WebError::BadRequest(json!("email_code_required")))?;
        util::email::consume_verification_code(
            &s,
            &verification_code_key("bind", operator.id, &email),
            code,
        )
        .await?;
    }

    let email = cds_db::email::create(&s.db.conn, operator.id, email, email_config.enabled).await?;

    Ok(Json(AddEmailResponse { email }))
}

/// Deletes email.
#[utoipa::path(
    delete,
    path = "/{mailbox}",
    tag = "user",
    params(
        ("mailbox" = String, Path, description = "Email address"),
    ),
    responses(
        (status = 200, description = "Removed", body = EmptyJson),
        (status = 409, description = "Last verified email", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_email"))]
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

/// Confirms ownership of a pending email address.
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
#[tracing::instrument(skip_all, fields(handler = "verify_email"))]
pub async fn verify_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
    ReqJson(body): ReqJson<EmailVerifyRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let email =
        cds_db::email::find_by_email::<cds_db::email::Model>(&s.db.conn, email.to_lowercase())
            .await?
            .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if !cds_db::get_config(&s.db.conn).await.email.enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    util::email::consume_verification_code(
        &s,
        &verification_code_key("verify", operator.id, &email.email),
        body.code.trim(),
    )
    .await?;
    cds_db::email::set_verified(&s.db.conn, operator.id, &email.email, true).await?;

    Ok(Json(EmptyJson::default()))
}

const VERIFY_RESEND_WAIT: Duration = Duration::from_secs(60);

fn verification_code_key(purpose: &str, user_id: i64, email: &str) -> String {
    format!("mailbox:{purpose}:{user_id}:{email}:code")
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct VerificationSendResponse {
    pub retry_after_seconds: u64,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SendBindEmailRequest {
    pub email: String,
}

/// Requests a verification code for an address before linking it.
#[utoipa::path(
    post,
    path = "/bind/send",
    tag = "user",
    request_body = SendBindEmailRequest,
    params(util::email::EmailLocaleQuery),
    responses(
        (status = 200, description = "Verification mail queued", body = VerificationSendResponse),
        (status = 400, description = "Invalid or disallowed email", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 409, description = "Email already linked", body = crate::traits::ErrorResponse),
        (status = 429, description = "Verification cooldown", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "send_bind_email"))]
pub async fn send_bind_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Query(locale): Query<util::email::EmailLocaleQuery>,
    ReqJson(body): ReqJson<SendBindEmailRequest>,
) -> Result<Json<VerificationSendResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let config = cds_db::get_config(&s.db.conn).await;
    if !config.email.enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email = body.email.to_lowercase();
    util::email::validate_new_address(&email, &config.email.whitelist)?;
    if !cds_db::user::is_email_unique(&s.db.conn, &email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    let key = verification_code_key("bind", operator.id, &email);
    queue_verification(
        &s,
        &operator,
        &email,
        &key,
        locale.locale.as_deref(),
        &config.meta.title,
    )
    .await
    .map(Json)
}

/// Sends an address-verification message through the mail queue.
#[utoipa::path(
    post,
    path = "/{mailbox}/verify/send",
    tag = "user",
    params(
        ("mailbox" = String, Path, description = "Email address"),
        util::email::EmailLocaleQuery,
    ),
    responses(
        (status = 200, description = "Verification mail queued", body = VerificationSendResponse),
        (status = 429, description = "Verification cooldown", body = crate::traits::ErrorResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "send_verify_email"))]
pub async fn send_verify_email(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
    Query(locale): Query<util::email::EmailLocaleQuery>,
) -> Result<Json<VerificationSendResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let config = cds_db::get_config(&s.db.conn).await;
    if !config.email.enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email: cds_db::email::Model =
        cds_db::email::find_by_email(&s.db.conn, email.to_lowercase())
            .await?
            .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    let key = verification_code_key("verify", operator.id, &email.email);
    queue_verification(
        &s,
        &operator,
        &email.email,
        &key,
        locale.locale.as_deref(),
        &config.meta.title,
    )
    .await
    .map(Json)
}

async fn queue_verification(
    s: &AppState,
    operator: &UserAccountView,
    email: &str,
    code_key: &str,
    locale: Option<&str>,
    title: &str,
) -> Result<VerificationSendResponse, WebError> {
    let code = nanoid!();
    let payload = Payload::transactional(
        operator.name.clone(),
        email.to_owned(),
        EmailPurpose::Verify,
        locale,
        &code,
        title,
    )?;

    let cooldown_key = format!("mailbox:verify:{email}:cooldown");
    let deadline = OffsetDateTime::now_utc().unix_timestamp() + VERIFY_RESEND_WAIT.as_secs() as i64;
    if !s
        .cache
        .set_if_absent(&cooldown_key, deadline, VERIFY_RESEND_WAIT)
        .await?
    {
        let expires_at = s.cache.get::<i64>(&cooldown_key).await?.unwrap_or_default();
        let retry_after_seconds = (expires_at - OffsetDateTime::now_utc().unix_timestamp()).max(1);
        return Err(WebError::TooManyRequests(json!({
            "code": "email_send_too_frequently",
            "retry_after_seconds": retry_after_seconds,
        })));
    }

    let result: Result<(), WebError> = async {
        s.cache.set_with_ttl(code_key, code, CODE_TTL).await?;
        s.queue.publish(SUBJECT, payload, Some(operator.id)).await?;
        Ok(())
    }
    .await;
    if result.is_err() {
        let _ = s.cache.delete_if_equal(&cooldown_key, deadline).await;
    }
    result?;

    Ok(VerificationSendResponse {
        retry_after_seconds: (deadline - OffsetDateTime::now_utc().unix_timestamp()).max(0) as u64,
    })
}

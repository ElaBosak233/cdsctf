//! HTTP API for user-owned personal access tokens.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
};
use cds_db::sea_orm::ActiveValue::Set;
use garde::Validate;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson},
    traits::{AppState, AuthMethod, AuthPrincipal, EmptyJson, WebError},
    util,
};

const MAX_EXPIRATION: time::Duration = time::Duration::days(365);

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_personal_access_tokens).with_state(state.clone()))
        .routes(routes!(create_personal_access_token).with_state(state.clone()))
        .routes(routes!(revoke_personal_access_token).with_state(state))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PersonalAccessTokenListResponse {
    pub personal_access_tokens: Vec<cds_db::PersonalAccessTokenView>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct CreatePersonalAccessTokenRequest {
    #[garde(length(min = 1, max = 64))]
    pub name: String,
    #[serde(with = "cds_db::time_format")]
    pub expires_at: time::OffsetDateTime,
    pub captcha: Option<cds_captcha::Answer>,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreatePersonalAccessTokenResponse {
    pub token: String,
    pub personal_access_token: cds_db::PersonalAccessTokenView,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "personal access token",
    responses(
        (status = 200, description = "Personal access tokens", body = PersonalAccessTokenListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn list_personal_access_tokens(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<PersonalAccessTokenListResponse>, WebError> {
    let operator = require_session(&ext)?;
    let tokens = cds_db::personal_access_token::find_by_user_id::<cds_db::PersonalAccessTokenView>(
        &s.db.conn,
        operator.id,
    )
    .await?;
    Ok(Json(PersonalAccessTokenListResponse {
        personal_access_tokens: tokens,
    }))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "personal access token",
    request_body = CreatePersonalAccessTokenRequest,
    responses(
        (status = 201, description = "Created personal access token", body = CreatePersonalAccessTokenResponse),
        (status = 400, description = "Invalid request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn create_personal_access_token(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<CreatePersonalAccessTokenRequest>,
) -> Result<
    (
        axum::http::StatusCode,
        Json<CreatePersonalAccessTokenResponse>,
    ),
    WebError,
> {
    let operator = require_session(&ext)?;
    if body.name.trim().is_empty() {
        return Err(WebError::BadRequest(json!("name_required")));
    }
    if !s
        .captcha
        .check(&cds_captcha::Answer {
            client_ip: Some(ext.client_ip.clone()),
            ..body.captcha.unwrap_or_default()
        })
        .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    let now = time::OffsetDateTime::now_utc();
    if body.expires_at <= now || body.expires_at > now + MAX_EXPIRATION {
        return Err(WebError::BadRequest(json!("invalid_expiration")));
    }

    let token_id = nanoid!(16);
    let secret = nanoid!(48);
    let token = format!("cds_pat_{token_id}.{secret}");
    let view = cds_db::personal_access_token::create::<cds_db::PersonalAccessTokenView>(
        &s.db.conn,
        cds_db::personal_access_token::ActiveModel {
            user_id: Set(operator.id),
            name: Set(body.name.trim().to_owned()),
            token_id: Set(token_id),
            token_hash: Set(util::crypto::hash_access_token(&secret)),
            expires_at: Set(body.expires_at),
            ..Default::default()
        },
    )
    .await?;

    Ok((
        axum::http::StatusCode::CREATED,
        Json(CreatePersonalAccessTokenResponse {
            token,
            personal_access_token: view,
        }),
    ))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "personal access token",
    params(("id" = i64, Path, description = "Personal access token id")),
    responses(
        (status = 200, description = "Revoked", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn revoke_personal_access_token(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = require_session(&ext)?;
    cds_db::personal_access_token::revoke(&s.db.conn, operator.id, id).await?;
    Ok(Json(EmptyJson::default()))
}

fn require_session(ext: &AuthPrincipal) -> Result<&cds_db::UserAccountView, WebError> {
    if ext.auth_method != AuthMethod::Session {
        return Err(WebError::Unauthorized(json!("session_required")));
    }
    ext.operator
        .as_ref()
        .ok_or(WebError::Unauthorized(json!("unauthorized")))
}

#[cfg(test)]
mod tests {
    use super::CreatePersonalAccessTokenRequest;

    #[test]
    fn create_request_accepts_rfc3339_expiration() {
        let request: CreatePersonalAccessTokenRequest = serde_json::from_value(serde_json::json!({
            "name": "deployment",
            "expires_at": "2027-01-01T00:00:00.000Z",
            "captcha": null,
        }))
        .expect("RFC3339 timestamps should deserialize");

        assert_eq!(request.name, "deployment");
        assert_eq!(request.expires_at.unix_timestamp(), 1_798_761_600);
    }
}

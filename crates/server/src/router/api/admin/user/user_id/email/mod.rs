use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Email,
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
    traits::{AppState, EmptySuccess, WebError},
};


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_email).with_state(state.clone()))
        .routes(routes!(add_email).with_state(state.clone()))
        .routes(routes!(delete_email).with_state(state.clone()))
        .routes(routes!(verify_email).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminEmailsListResponse {
    pub items: Vec<Email>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "Emails", body = AdminEmailsListResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_email(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
) -> Result<Json<AdminEmailsListResponse>, WebError> {
    let emails = cds_db::email::find_by_user_id(&s.db.conn, user_id).await?;
    let total = emails.len() as u64;
    Ok(Json(AdminEmailsListResponse { items: emails, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct AdminAddEmailRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminEmailResponse {
    pub email: Email,
}

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
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn add_email(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
    ReqJson(body): ReqJson<AdminAddEmailRequest>,
) -> Result<Json<AdminEmailResponse>, WebError> {
    let email = cds_db::email::create::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(user_id),
            email: Set(body.email.to_lowercase()),
            verified: Set(true),
        },
    )
    .await?;

    Ok(Json(AdminEmailResponse { email }))
}

#[utoipa::path(
    delete,
    path = "/{mailbox}",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
        ("mailbox" = String, Path, description = "Email"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn delete_email(
    State(s): State<Arc<AppState>>,
    Path((user_id, email)): Path<(i64, String)>,
) -> Result<Json<EmptySuccess>, WebError> {
    let email = email.to_lowercase();
    let _ = cds_db::email::delete(&s.db.conn, user_id, email).await?;
    Ok(Json(EmptySuccess::default()))
}

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
        (status = 400, description = "Bad request", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn verify_email(
    State(s): State<Arc<AppState>>,
    Path((user_id, email)): Path<(i64, String)>,
) -> Result<Json<AdminEmailResponse>, WebError> {
    let email = cds_db::email::find_by_email::<Email>(&s.db.conn, email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != user_id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    let email = cds_db::email::update::<Email>(
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

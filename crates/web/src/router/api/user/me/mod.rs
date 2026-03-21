mod avatar;
mod email;
mod note;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::sea_orm::{
    ActiveValue::{Set, Unchanged},
    NotSet,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use super::UserResponse;
use crate::{
    extract::{Extension, Json as ReqJson},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util,
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_user_profile).with_state(state.clone()))
        .routes(routes!(update_user_profile).with_state(state.clone()))
        .routes(routes!(delete_user_profile).with_state(state.clone()))
        .routes(routes!(update_user_profile_password).with_state(state.clone()))
        .nest("/emails", email::router(state.clone()))
        .nest("/avatar", avatar::router(state.clone()))
        .nest("/notes", note::router(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    responses(
        (status = 200, description = "Current user", body = UserResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_user_profile(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<UserResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let user = cds_db::user::find_by_id::<cds_db::User>(&s.db.conn, operator.id)
        .await?
        .ok_or(WebError::NotFound(json!("")))?;
    Ok(Json(UserResponse { user }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateUserProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    put,
    path = "/",
    tag = "user",
    request_body = UpdateUserProfileRequest,
    responses(
        (status = 200, description = "Updated user", body = UserResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn update_user_profile(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<UpdateUserProfileRequest>,
) -> Result<Json<UserResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let user = cds_db::user::update::<cds_db::User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            id: Unchanged(operator.id),
            name: body.name.map_or(NotSet, Set),
            description: body.description.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(UserResponse { user }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DeleteUserProfileRequest {
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "user",
    request_body = DeleteUserProfileRequest,
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn delete_user_profile(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<DeleteUserProfileRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    if !s
        .captcha
        .check(&cds_captcha::Answer {
            client_ip: Some(ext.client_ip),
            ..body.captcha.unwrap_or_default()
        })
        .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    let hashed_password = operator.hashed_password.clone();

    if !util::crypto::verify_password(body.password, hashed_password) {
        return Err(WebError::BadRequest(json!("password_invalid")));
    }

    cds_db::user::delete(&s.db.conn, operator.id).await?;

    Ok(Json(EmptyJson::default()))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateUserProfilePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[utoipa::path(
    put,
    path = "/password",
    tag = "user",
    request_body = UpdateUserProfilePasswordRequest,
    responses(
        (status = 200, description = "Password updated", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn update_user_profile_password(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<UpdateUserProfilePasswordRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let hashed_password = operator.hashed_password.clone();

    if !util::crypto::verify_password(body.old_password, hashed_password) {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    let hashed_password = util::crypto::hash_password(body.new_password);

    cds_db::user::update_password(&s.db.conn, operator.id, hashed_password).await?;

    Ok(Json(EmptyJson::default()))
}

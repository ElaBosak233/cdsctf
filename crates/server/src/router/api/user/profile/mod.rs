mod avatar;
mod email;

use axum::{Router, http::StatusCode};
use cds_db::{
    User,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json},
    traits::{AuthPrincipal, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user_profile))
        .route("/", axum::routing::put(update_user_profile))
        .route("/", axum::routing::delete(delete_user_profile))
        .route(
            "/password",
            axum::routing::put(update_user_profile_password),
        )
        .nest("/emails", email::router())
        .nest("/avatar", avatar::router())
}

pub async fn get_user_profile(
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<WebResponse<User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let user = cds_db::user::find_by_id::<User>(operator.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: user,
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn update_user_profile(
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<UpdateUserProfileRequest>,
) -> Result<WebResponse<User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let user = cds_db::user::update::<User>(cds_db::user::ActiveModel {
        id: Unchanged(operator.id),
        name: body.name.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteUserProfileRequest {
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn delete_user_profile(
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<DeleteUserProfileRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    if !cds_captcha::check(&cds_captcha::Answer {
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

    cds_db::user::delete(operator.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfilePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn update_user_profile_password(
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<UpdateUserProfilePasswordRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let hashed_password = operator.hashed_password.clone();

    if !util::crypto::verify_password(body.old_password, hashed_password) {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    let hashed_password = util::crypto::hash_password(body.new_password);

    cds_db::user::update_password(operator.id, hashed_password).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

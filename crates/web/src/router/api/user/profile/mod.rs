mod avatar;
pub mod verify;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode};
use cds_db::{
    get_db,
    sea_orm::{
        ActiveModelTrait,
        ActiveValue::{Set, Unchanged},
        EntityTrait, NotSet,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json},
    traits::{Ext, WebError, WebResponse},
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
        .nest("/avatar", avatar::router())
        .nest("/verify", verify::router())
}

pub async fn get_user_profile(
    Extension(ext): Extension<Ext>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(operator),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfileRequest {
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub description: Option<String>,
}

pub async fn update_user_profile(
    Extension(ext): Extension<Ext>,
    Json(mut body): Json<UpdateUserProfileRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    if let Some(email) = body.email {
        body.email = Some(email.to_lowercase());
        if !cds_db::util::is_user_email_unique(operator.id, &email.to_lowercase()).await? {
            return Err(WebError::Conflict(json!("email_already_exists")));
        }
    }

    let is_verified = body
        .email
        .as_ref()
        .map(|email| {
            if email != &operator.email {
                false
            } else {
                operator.is_verified
            }
        })
        .unwrap_or(operator.is_verified);

    let user = cds_db::entity::user::ActiveModel {
        id: Unchanged(operator.id),
        nickname: body.nickname.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        is_verified: if is_verified != operator.is_verified {
            Set(is_verified)
        } else {
            Unchanged(operator.is_verified)
        },
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

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
    Extension(ext): Extension<Ext>,
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

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("password_invalid")));
    }

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(operator.id),
        username: Set(format!("[DELETED]_{}", operator.username)),
        email: Set(format!("deleted_{}@del.cdsctf", operator.email)),
        deleted_at: Set(Some(chrono::Utc::now().timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

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
    Extension(ext): Extension<Ext>,
    Json(body): Json<UpdateUserProfilePasswordRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let hashed_password = operator.hashed_password.clone();

    if Argon2::default()
        .verify_password(
            body.old_password.as_bytes(),
            &PasswordHash::new(&hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    let hashed_password = Argon2::default()
        .hash_password(
            body.new_password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )
        .unwrap()
        .to_string();

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(operator.id),
        hashed_password: Set(hashed_password),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

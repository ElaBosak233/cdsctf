mod user_id;

use std::str::FromStr;

use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode, response::IntoResponse};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::VJson,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_user))
        .nest("/{user_id}", user_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create_user(
    VJson(mut body): VJson<CreateUserRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    body.email = body.email.to_lowercase();
    if !cds_db::util::is_user_email_unique(0, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::util::is_user_username_unique(0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        group: Set(body.group),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let mut user = cds_db::transfer::User::from(user);

    user.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}

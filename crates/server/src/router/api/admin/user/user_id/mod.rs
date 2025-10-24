use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode};
use cds_db::{
    User,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    user::Group,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .route("/", axum::routing::put(update_user))
        .route("/", axum::routing::delete(delete_user))
}

pub async fn get_user(Path(user_id): Path<i64>) -> Result<WebResponse<User>, WebError> {
    let user = crate::util::loader::prepare_user(user_id).await?;

    Ok(WebResponse {
        data: Some(user),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
    pub description: Option<String>,
}

/// Update a user with given data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_user(
    Path(user_id): Path<i64>,
    VJson(mut body): VJson<UpdateUserRequest>,
) -> Result<WebResponse<User>, WebError> {
    let user = crate::util::loader::prepare_user(user_id).await?;

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = cds_db::user::update::<User>(cds_db::user::ActiveModel {
        id: Unchanged(user.id),
        name: body.name.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        group: body.group.map_or(NotSet, Set),
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

/// Delete a user with given data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_user(Path(user_id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    cds_db::user::delete(user_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

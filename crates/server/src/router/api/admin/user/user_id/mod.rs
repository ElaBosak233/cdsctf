mod email;

use std::sync::Arc;

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State};
use cds_db::{
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    user::Group,
};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    router::api::user::UserResponse,
    traits::{AppState, EmptySuccess, WebError},
};


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_user).with_state(state.clone()))
        .routes(routes!(update_user).with_state(state.clone()))
        .routes(routes!(delete_user).with_state(state.clone()))
        .nest("/emails", email::openapi_router(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "User", body = UserResponse),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_user(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, WebError> {
    let user = crate::util::loader::prepare_user(&s.db.conn, user_id).await?;
    Ok(Json(UserResponse { user }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
    pub description: Option<String>,
}

#[utoipa::path(
    put,
    path = "/",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Updated user", body = UserResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn update_user(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
    VJson(mut body): VJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>, WebError> {
    let user = crate::util::loader::prepare_user(&s.db.conn, user_id).await?;

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = cds_db::user::update(
        &s.db.conn,
        cds_db::user::ActiveModel {
            id: Unchanged(user.id),
            name: body.name.map_or(NotSet, Set),
            hashed_password: body.password.map_or(NotSet, Set),
            group: body.group.map_or(NotSet, Set),
            description: body.description.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(UserResponse { user }))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn delete_user(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
) -> Result<Json<EmptySuccess>, WebError> {
    cds_db::user::delete(&s.db.conn, user_id).await?;
    Ok(Json(EmptySuccess::default()))
}

//! HTTP routing for `user` — Axum router wiring and OpenAPI route registration.

/// Defines the `user_id` submodule (see sibling `*.rs` files).
mod user_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    Email, User,
    sea_orm::ActiveValue::Set,
    user::{FindUserOptions, Group},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Query, VJson},
    router::api::user::UserResponse,
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_users).with_state(state.clone()))
        .routes(routes!(create_user).with_state(state.clone()))
        .nest("/{user_id}", user_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetUsersRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminUsersListResponse {
    pub users: Vec<User>,
    pub total: u64,
}

/// Returns users.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-user",
    params(GetUsersRequest),
    responses(
        (status = 200, description = "Users", body = AdminUsersListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_users"))]
pub async fn get_users(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetUsersRequest>,
) -> Result<Json<AdminUsersListResponse>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (users, total) = cds_db::user::find(
        &s.db.conn,
        FindUserOptions {
            id: params.id,
            name: params.name,
            group: params.group,
            sorts: params.sorts,
            page: Some(page),
            size: Some(size),
        },
    )
    .await?;

    Ok(Json(AdminUsersListResponse { users, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub group: Group,
}

/// Creates user.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-user",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "Created user", body = UserResponse),
        (status = 409, description = "Conflict", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_user"))]
pub async fn create_user(
    State(s): State<Arc<AppState>>,
    VJson(mut body): VJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), WebError> {
    body.username = body.username.to_lowercase();
    if !cds_db::user::is_username_unique(&s.db.conn, 0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = crate::util::crypto::hash_password(body.password);

    let user = cds_db::user::create::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            name: Set(body.name),
            username: Set(body.username),
            hashed_password: Set(hashed_password),
            group: Set(body.group),
            ..Default::default()
        },
    )
    .await?;

    let _ = cds_db::email::create::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(user.id),
            email: Set(body.email),
            verified: Set(true),
        },
    )
    .await?;

    info!(
        user_id = user.id,
        username = %user.username,
        group = ?user.group,
        "admin created user"
    );

    Ok((StatusCode::CREATED, Json(UserResponse { user })))
}

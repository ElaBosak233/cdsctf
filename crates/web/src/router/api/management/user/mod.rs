//! HTTP routing for `user` — Axum router wiring and OpenAPI route registration.

/// Defines the `user_id` submodule (see sibling `*.rs` files).
mod user_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    Permission, UserAccountView,
    sea_orm::{
        ActiveValue::{NotSet, Set},
        TransactionTrait,
    },
    user::{FindUserOptions, PermissionFilter},
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Query, VJson},
    middleware::auth::MANAGEMENT_PERMISSIONS,
    router::api::user::UserResponse,
    traits::{AppState, WebError},
    util::email::validate_new_address,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_users).with_state(state.clone()))
        .routes(routes!(create_user).with_state(state.clone()))
        .nest("/{user_id}", user_id::router(state.clone()))
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ManagementAccess {
    Management,
    NoManagement,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetUsersRequest {
    /// Searches display names and login names, or matches an exact positive
    /// #ID.
    pub q: Option<String>,
    pub banned: Option<bool>,
    /// Filters accounts by the presence of management permissions.
    pub access: Option<ManagementAccess>,
    /// Requires a specific permission. Can be used in place of access.
    pub permission: Option<Permission>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminUsersListResponse {
    pub users: Vec<UserAccountView>,
    pub total: u64,
}

/// Returns users.
#[utoipa::path(
    get,
    path = "/",
    tag = "management-user",
    params(GetUsersRequest),
    responses(
        (status = 200, description = "Users", body = AdminUsersListResponse),
        (status = 400, description = "Invalid filters", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_users"))]
pub async fn get_users(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetUsersRequest>,
) -> Result<Json<AdminUsersListResponse>, WebError> {
    let page = params.page.unwrap_or(1).max(1);
    let size = params.size.unwrap_or(10).clamp(1, 100);
    let permissions = match (params.access, params.permission) {
        (Some(_), Some(_)) => return Err(WebError::BadRequest(json!("invalid_query"))),
        (Some(ManagementAccess::Management), None) => {
            Some(PermissionFilter::AnyOf(MANAGEMENT_PERMISSIONS.to_vec()))
        }
        (Some(ManagementAccess::NoManagement), None) => {
            Some(PermissionFilter::NoneOf(MANAGEMENT_PERMISSIONS.to_vec()))
        }
        (None, Some(permission)) => Some(PermissionFilter::AnyOf(vec![permission])),
        (None, None) => None,
    };
    let keyword = params.q.as_deref().map(str::trim).filter(|q| !q.is_empty());
    let search_id = keyword
        .and_then(|q| q.strip_prefix('#'))
        .filter(|id| id.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|id| id.parse::<i64>().ok())
        .filter(|&id| id > 0);

    let (users, total) = cds_db::user::find(
        &s.db.conn,
        FindUserOptions {
            id: search_id,
            keyword: keyword.filter(|_| search_id.is_none()).map(str::to_owned),
            banned: params.banned,
            permissions,
            sorts: params.sorts,
            page: Some(page),
            size: Some(size),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(AdminUsersListResponse { users, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct CreateUserRequest {
    pub name: String,
    #[garde(length(min = 3, max = 20))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    pub password: String,
    pub banned: Option<bool>,
    pub permissions: Option<Vec<Permission>>,
}

/// Creates user.
#[utoipa::path(
    post,
    path = "/",
    tag = "management-user",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "Created user", body = UserResponse),
        (status = 400, description = "Invalid or disallowed email", body = crate::traits::ErrorResponse),
        (status = 409, description = "Conflict", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_user"))]
pub async fn create_user(
    State(s): State<Arc<AppState>>,
    VJson(mut body): VJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), WebError> {
    let email_config = cds_db::get_config(&s.db.conn).await.email;
    validate_new_address(&body.email, &email_config.whitelist)?;
    body.username = body.username.to_lowercase();
    if !cds_db::user::is_username_unique(&s.db.conn, 0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = crate::util::crypto::hash_password(body.password);
    let transaction = s.db.conn.begin().await.map_err(cds_db::DbError::from)?;
    let user = cds_db::user::create::<UserAccountView>(
        &transaction,
        cds_db::user::ActiveModel {
            name: Set(body.name),
            username: Set(body.username),
            hashed_password: Set(hashed_password),
            banned: body.banned.map_or(NotSet, Set),
            permissions: Set(body.permissions.unwrap_or_default()),
            ..Default::default()
        },
    )
    .await?;

    cds_db::email::create(&transaction, user.id, body.email.to_lowercase(), false).await?;
    transaction.commit().await.map_err(cds_db::DbError::from)?;

    info!(
        user_id = user.id,
        username = %user.username,
        banned = user.banned,
        permissions = ?user.permissions,
        "management created user"
    );

    Ok((StatusCode::CREATED, Json(UserResponse { user })))
}

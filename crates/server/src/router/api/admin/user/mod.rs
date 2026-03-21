mod user_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Email, User,
    sea_orm::ActiveValue::Set,
    user::{FindUserOptions, Group},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_users))
        .route("/", axum::routing::post(create_user))
        .nest("/{user_id}", user_id::router())
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_users).with_state(state.clone()))
        .routes(routes!(create_user).with_state(state.clone()))
        .nest("/{user_id}", user_id::openapi_router(state.clone()))
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
    pub items: Vec<User>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-user",
    params(GetUsersRequest),
    responses(
        (status = 200, description = "Users", body = AdminUsersListResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
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

    Ok(Json(AdminUsersListResponse { items: users, total }))
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

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-user",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "Created user", body = UserResponse),
        (status = 409, description = "Conflict", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn create_user(
    State(s): State<Arc<AppState>>,
    VJson(mut body): VJson<CreateUserRequest>,
) -> Result<Json<UserResponse>, WebError> {
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

    Ok(Json(UserResponse { user }))
}

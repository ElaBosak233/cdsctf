use std::sync::Arc;

use axum::{Json, Router, extract::State};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

use super::UserResponse;

mod avatar;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .nest("/avatar", avatar::router())
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_user).with_state(state.clone()))
        .nest(
            "/avatar",
            OpenApiRouter::from(Router::new().with_state(state.clone())).routes(
                routes!(avatar::get_user_avatar).with_state(state.clone()),
            ),
        )
}

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "User profile", body = UserResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ApiJsonError),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_user(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let user = cds_db::user::find_by_id::<cds_db::User>(&s.db.conn, user_id)
        .await?
        .ok_or(WebError::NotFound(json!("")))?;
    Ok(Json(UserResponse { user }))
}

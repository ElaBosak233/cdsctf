//! HTTP routing for `user_id` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::UserProfile;
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use super::UserPublicResponse;
use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Defines the `avatar` submodule (see sibling `*.rs` files).
mod avatar;

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_user).with_state(state.clone()))
        .routes(
            routes!(crate::router::api::admin::user::user_id::update_user)
                .with_state(state.clone()),
        )
        .routes(
            routes!(crate::router::api::admin::user::user_id::delete_user)
                .with_state(state.clone()),
        )
        .nest(
            "/emails",
            crate::router::api::admin::user::user_id::email::router(state.clone()),
        )
        .nest(
            "/avatar",
            OpenApiRouter::from(Router::new().with_state(state.clone()))
                .routes(routes!(avatar::get_user_avatar).with_state(state.clone())),
        )
}

/// Returns user.
#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "User profile", body = UserPublicResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_user"))]
pub async fn get_user(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(user_id): Path<i64>,
) -> Result<Json<serde_json::Value>, WebError> {
    let user = cds_db::user::find_by_id::<cds_db::UserAccountView>(&s.db.conn, user_id)
        .await?
        .ok_or(WebError::NotFound(json!("not_found")))?;
    if principal
        .operator
        .as_ref()
        .is_some_and(|operator| operator.group >= cds_db::user::Group::Admin)
    {
        return Ok(Json(serde_json::json!({ "user": user })));
    }
    Ok(Json(serde_json::json!({
        "user": UserProfile::from(&user)
    })))
}

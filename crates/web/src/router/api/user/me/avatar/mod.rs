//! HTTP routing for `avatar` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::{
    User,
    sea_orm::{Set, Unchanged},
};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use serde_json::json;

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util::media::handle_multipart,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(
            routes!(save_user_avatar)
                .with_state(state.clone())
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .routes(routes!(delete_user_avatar).with_state(state.clone()))
}

/// Stores the authenticated user's avatar object.
#[utoipa::path(
    post,
    path = "/",
    tag = "user",
    responses(
        (status = 200, description = "Avatar saved", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "save_user_avatar"))]
pub async fn save_user_avatar(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let hash = cds_media::util::hash(data.clone());

    s.media.save("media".to_owned(), hash.clone(), data).await?;

    let _ = cds_db::user::update::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            id: Unchanged(operator.id),
            avatar_hash: Set(Some(hash)),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

/// Deletes user avatar.
#[utoipa::path(
    delete,
    path = "/",
    tag = "user",
    responses(
        (status = 200, description = "Avatar removed", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_user_avatar"))]
pub async fn delete_user_avatar(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let user = cds_db::user::find_by_id::<cds_db::user::Model>(&s.db.conn, operator.id)
        .await?
        .ok_or(WebError::NotFound(json!("")))?;

    if let Some(hash) = user.avatar_hash {
        s.media.delete("media".to_owned(), hash).await?;
    }

    let _ = cds_db::user::update::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            id: Unchanged(operator.id),
            avatar_hash: Set(None),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

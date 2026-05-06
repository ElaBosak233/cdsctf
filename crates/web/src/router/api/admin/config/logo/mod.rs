//! HTTP routing for `logo` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
};
use cds_db::config::{get as get_config, save as save_config};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    traits::{AppState, EmptyJson, WebError},
    util::media::handle_multipart,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(save_logo).with_state(state.clone()))
        .routes(routes!(delete_logo).with_state(state.clone()))
}

/// Stores the public site logo in object storage.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-config",
    responses(
        (status = 200, description = "Logo saved", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "save_logo"))]
pub async fn save_logo(
    State(s): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let hash = cds_media::util::hash(data.clone());

    s.media.save("media".to_owned(), hash.clone(), data).await?;

    let mut config = get_config(&s.db.conn).await?;
    config.logo_hash = Some(hash);
    save_config(&s.db.conn, config).await?;

    Ok(Json(EmptyJson::default()))
}

/// Deletes logo.
#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-config",
    responses(
        (status = 200, description = "Logo removed", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_logo"))]
pub async fn delete_logo(State(s): State<Arc<AppState>>) -> Result<Json<EmptyJson>, WebError> {
    let mut config = get_config(&s.db.conn).await?;

    if let Some(hash) = config.logo_hash.take() {
        s.media.delete("media".to_owned(), hash).await?;
    }

    save_config(&s.db.conn, config).await?;

    Ok(Json(EmptyJson::default()))
}

//! HTTP routing for `logo` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::config::{self, get as get_config};
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
        .routes(
            routes!(save_logo)
                .with_state(state.clone())
                .layer(DefaultBodyLimit::max(5 * 1024 * 1024 /* MB */)),
        )
        .routes(routes!(delete_logo).with_state(state.clone()))
}

/// Stores the public site logo in object storage.
#[utoipa::path(
    post,
    path = "/",
    tag = "management-config",
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

    config::set_logo_hash(&s.db.conn, Some(hash)).await?;

    Ok(Json(EmptyJson::default()))
}

/// Deletes logo.
#[utoipa::path(
    delete,
    path = "/",
    tag = "management-config",
    responses(
        (status = 200, description = "Logo removed", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_logo"))]
pub async fn delete_logo(State(s): State<Arc<AppState>>) -> Result<Json<EmptyJson>, WebError> {
    let config = get_config(&s.db.conn).await?;
    if let Some(hash) = config.logo_hash {
        s.media.delete("media".to_owned(), hash).await?;
        config::set_logo_hash(&s.db.conn, None).await?;
    }

    Ok(Json(EmptyJson::default()))
}

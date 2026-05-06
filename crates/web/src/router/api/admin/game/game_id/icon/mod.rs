//! HTTP routing for `icon` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::{
    Game,
    sea_orm::{Set, Unchanged},
};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use serde_json::json;

use crate::{
    extract::Path,
    traits::{AppState, EmptyJson, WebError},
    util::media::handle_multipart,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(
            routes!(save_game_icon)
                .with_state(state.clone())
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .routes(routes!(delete_game_icon).with_state(state.clone()))
}

/// Stores a competition icon image.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Icon saved", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "save_game_icon"))]
pub async fn save_game_icon(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let hash = cds_media::util::hash(data.clone());

    s.media.save("media".to_owned(), hash.clone(), data).await?;

    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            icon_hash: Set(Some(hash)),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

/// Deletes game icon.
#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Icon removed", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_game_icon"))]
pub async fn delete_game_icon(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let game = cds_db::game::find_by_id::<cds_db::Game>(&s.db.conn, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("game_not_found")))?;

    if let Some(hash) = game.icon_hash {
        s.media.delete("media".to_owned(), hash).await?;
    }

    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            icon_hash: Set(None),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

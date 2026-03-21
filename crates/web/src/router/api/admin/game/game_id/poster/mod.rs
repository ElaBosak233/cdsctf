//! HTTP routing for `poster` — Axum router wiring and OpenAPI route
//! registration.

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

use crate::{
    extract::Path,
    traits::{AppState, EmptyJson, WebError},
    util::media::handle_multipart,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(
            routes!(save_game_poster)
                .with_state(state.clone())
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .routes(routes!(delete_game_poster).with_state(state.clone()))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Poster saved", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Stores a game poster image.
pub async fn save_game_poster(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let path = format!("games/{}", game_id);

    s.media.save(path, "poster".to_owned(), data).await?;

    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            has_poster: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Poster removed", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Deletes game poster.
pub async fn delete_game_poster(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let path = format!("games/{}", game_id);

    s.media.delete(path, "poster".to_owned()).await?;

    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            has_poster: Set(false),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

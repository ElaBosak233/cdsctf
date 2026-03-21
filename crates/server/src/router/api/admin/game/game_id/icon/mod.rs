use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
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
    traits::{AppState, EmptySuccess, WebError},
    util::media::handle_multipart,
};


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(save_game_icon).with_state(state.clone()))
        .routes(routes!(delete_game_icon).with_state(state.clone()))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Icon saved", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn save_game_icon(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<Json<EmptySuccess>, WebError> {
    let data = handle_multipart(multipart, mime::IMAGE).await?;

    let path = format!("games/{}", game_id);

    s.media.save(path, "icon".to_owned(), data).await?;

    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            has_icon: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptySuccess::default()))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Icon removed", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn delete_game_icon(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptySuccess>, WebError> {
    let path = format!("games/{}", game_id);

    s.media.delete(path, "icon".to_owned()).await?;

    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            has_icon: Set(false),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptySuccess::default()))
}

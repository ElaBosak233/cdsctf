use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::{
    Game,
    sea_orm::{Set, Unchanged},
};

use crate::{
    extract::Path,
    traits::{AppState, WebError, WebResponse},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_game_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_game_poster))
}

pub async fn save_game_poster(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let path = format!("games/{}/poster", game_id);

    let _ = util::media::save_img(s.media.clone(), path, multipart).await;
    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            has_poster: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}

pub async fn delete_game_poster(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let path = format!("games/{}/poster", game_id);

    let _ = util::media::delete_img(s.media.clone(), path).await;
    let _ = cds_db::game::update::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game_id),
            has_poster: Set(false),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}

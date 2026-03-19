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
    util::media::handle_multipart,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_game_icon)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_game_icon))
}

pub async fn save_game_icon(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
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

    Ok(WebResponse::default())
}

pub async fn delete_game_icon(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
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

    Ok(WebResponse::default())
}

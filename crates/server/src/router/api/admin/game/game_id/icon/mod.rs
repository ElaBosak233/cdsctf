use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
};
use cds_db::{
    Game,
    sea_orm::{Set, Unchanged},
};

use crate::{
    extract::Path,
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_game_icon)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_game_icon))
}

pub async fn save_game_icon(
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let path = format!("games/{}/icon", game_id);

    let _ = util::media::save_img(path, multipart).await;
    let _ = cds_db::game::update::<Game>(cds_db::game::ActiveModel {
        id: Unchanged(game_id),
        has_icon: Set(true),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse::default())
}

pub async fn delete_game_icon(Path(game_id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    let path = format!("games/{}/icon", game_id);

    let _ = util::media::delete_img(path).await;

    let _ = cds_db::game::update::<Game>(cds_db::game::ActiveModel {
        id: Unchanged(game_id),
        has_icon: Set(false),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse::default())
}

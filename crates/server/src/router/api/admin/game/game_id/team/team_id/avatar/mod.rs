use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
};

use crate::{
    extract::Path,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_team_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_team_avatar))
}

pub async fn save_team_avatar(
    Path((game_id, team_id)): Path<(i64, i64)>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);

    crate::util::media::save_img(path, multipart).await
}

pub async fn delete_team_avatar(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);

    crate::util::media::delete_img(path).await
}

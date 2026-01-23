use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State},
};

use crate::{
    extract::Path,
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_team_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_team_avatar))
}

pub async fn save_team_avatar(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);

    crate::util::media::save_img(s.media.clone(), path, multipart).await
}

pub async fn delete_team_avatar(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);

    crate::util::media::delete_img(s.media.clone(), path).await
}

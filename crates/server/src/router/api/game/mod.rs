mod game_id;

use axum::{Router, http::StatusCode};
use cds_db::{GameMini, game::FindGameOptions};
use serde::{Deserialize, Serialize};

use crate::{
    extract::Query,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game))
        .nest("/{game_id}", game_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Get games with given params.
pub async fn get_game(
    Query(params): Query<GetGameRequest>,
) -> Result<WebResponse<Vec<GameMini>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(20);

    let (games, total) = cds_db::game::find(FindGameOptions {
        id: params.id,
        title: params.title,
        page: Some(page),
        size: Some(size),
        sorts: params.sorts,
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(games),
        total: Some(total),
        ..Default::default()
    })
}

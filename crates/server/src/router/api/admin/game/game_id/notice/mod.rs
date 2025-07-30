use axum::{Router, http::StatusCode};
use cds_db::{GameNotice, sea_orm::ActiveValue::Set};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Json, Path},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_game_notice))
        .route("/{notice_id}", axum::routing::delete(delete_game_notice))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameNoticeRequest {
    pub game_id: Option<i64>,
    pub title: String,
    pub content: String,
}

pub async fn create_game_notice(
    Path(game_id): Path<i64>,
    Json(body): Json<CreateGameNoticeRequest>,
) -> Result<WebResponse<GameNotice>, WebError> {
    let game_notice = cds_db::game_notice::create::<GameNotice>(cds_db::game_notice::ActiveModel {
        game_id: Set(game_id),
        title: Set(body.title),
        content: Set(body.content),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_notice),
        ..Default::default()
    })
}

pub async fn delete_game_notice(
    Path((game_id, notice_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    cds_db::game_notice::delete(notice_id, game_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

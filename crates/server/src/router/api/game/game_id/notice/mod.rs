use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::GameNotice;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_game_notice))
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct GetGameNoticeRequest {
//     pub game_id: Option<i64>,
// }

pub async fn get_game_notice(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Vec<GameNotice>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game_notices = cds_db::game_notice::find_by_game_id(&s.db.conn, game_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_notices),
        ..Default::default()
    })
}

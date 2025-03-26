use axum::{Router, http::StatusCode};
use cds_db::{
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter},
    transfer::GameNotice,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_game_notice))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameNoticeRequest {
    pub game_id: Option<i64>,
}

pub async fn get_game_notice(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
) -> Result<WebResponse<Vec<GameNotice>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game_notices = cds_db::entity::game_notice::Entity::find()
        .filter(cds_db::entity::game_notice::Column::GameId.eq(game_id))
        .all(get_db())
        .await?
        .into_iter()
        .map(cds_db::transfer::GameNotice::from)
        .collect::<Vec<GameNotice>>();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_notices),
        ..Default::default()
    })
}

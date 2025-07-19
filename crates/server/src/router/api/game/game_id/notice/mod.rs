use axum::{Router, http::StatusCode};
use cds_db::{
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::game_notice::GameNotice,
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_game_notice))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameNoticeRequest {
    pub game_id: Option<i64>,
}

pub async fn get_game_notice(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Vec<GameNotice>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game_notices = cds_db::entity::game_notice::Entity::find()
        .filter(cds_db::entity::game_notice::Column::GameId.eq(game_id))
        .into_model::<GameNotice>()
        .all(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_notices),
        ..Default::default()
    })
}

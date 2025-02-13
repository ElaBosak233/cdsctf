use axum::{Router, http::StatusCode};
use cds_db::{entity::user::Group, get_db, transfer::GameNotice};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game_notice))
        .route("/", axum::routing::post(create_game_notice))
        .route("/{notice_id}", axum::routing::delete(delete_game_notice))
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
        .map(|game_notice| cds_db::transfer::GameNotice::from(game_notice))
        .collect::<Vec<GameNotice>>();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_notices),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameNoticeRequest {
    pub game_id: Option<i64>,
    pub title: String,
    pub content: String,
}

pub async fn create_game_notice(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
    Json(body): Json<CreateGameNoticeRequest>,
) -> Result<WebResponse<GameNotice>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_notice = cds_db::entity::game_notice::ActiveModel {
        game_id: Set(game_id),
        title: Set(body.title),
        content: Set(body.content),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let game_notice = cds_db::transfer::GameNotice::from(game_notice);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_notice),
        ..Default::default()
    })
}

pub async fn delete_game_notice(
    Extension(ext): Extension<Ext>, Path((game_id, notice_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::game_notice::Entity::delete_many()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_notice::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_notice::Column::Id.eq(notice_id)),
        )
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

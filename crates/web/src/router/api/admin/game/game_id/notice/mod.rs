use axum::{Router, http::StatusCode};
use cds_db::{
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    model::game_notice::GameNotice,
    traits::{Ext, WebError, WebResponse},
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
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
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

    let game_notice = cds_db::entity::game_notice::Entity::find_by_id(game_notice.id)
        .into_model::<GameNotice>()
        .one(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: game_notice,
        ..Default::default()
    })
}

pub async fn delete_game_notice(
    Extension(ext): Extension<Ext>,
    Path((game_id, notice_id)): Path<(i64, i64)>,
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
        code: StatusCode::OK,
        ..Default::default()
    })
}

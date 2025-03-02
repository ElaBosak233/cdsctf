use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    response::IntoResponse,
};
use cds_db::get_db;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team_avatar))
        .route("/metadata", axum::routing::get(get_team_avatar_metadata))
        .route(
            "/",
            axum::routing::post(save_team_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_team_avatar))
}

pub async fn get_team_avatar(Path((game_id, team_id)): Path<(i64, i64)>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::get_img(path).await
}

pub async fn get_team_avatar_metadata(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::get_img_metadata(path).await
}

/// Save an avatar for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn save_team_avatar(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::save_img(path, multipart).await
}

/// Delete avatar for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn delete_team_avatar(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::delete_img(path).await
}

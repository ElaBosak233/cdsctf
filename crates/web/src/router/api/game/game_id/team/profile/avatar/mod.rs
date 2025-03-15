use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    response::IntoResponse,
};
use cds_db::{
    entity::user::Group,
    get_db,
    transfer::{Game, Team},
};
use sea_orm::{ColumnTrait, EntityTrait};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
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

/// Save an avatar for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn save_team_avatar(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(game_id, operator.id).await?;
    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);

    util::media::save_img(path, multipart).await
}

/// Delete avatar for the team.
pub async fn delete_team_avatar(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(game_id, operator.id).await?;
    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);

    util::media::delete_img(path).await
}

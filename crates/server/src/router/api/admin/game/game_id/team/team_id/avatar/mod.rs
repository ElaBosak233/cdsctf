use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
};
use cds_db::entity::user::Group;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
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
    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if operator.group != Group::Admin
        && !cds_db::util::is_user_in_team(operator.id, team.id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    crate::util::media::save_img(path, multipart).await
}

/// Delete avatar for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn delete_team_avatar(
    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if operator.group != Group::Admin
        && !cds_db::util::is_user_in_team(operator.id, team.id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    crate::util::media::delete_img(path).await
}

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
};
use serde_json::json;
use cds_db::sea_orm::{Set, Unchanged};
use cds_db::Team;
use crate::{
    extract::{Extension, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
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
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(game_id, operator.id).await?;
    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);
    let _ = util::media::save_img(path, multipart).await?;

    let _ = cds_db::team::update::<Team>(cds_db::team::ActiveModel {
        id: Unchanged(team.id),
        has_avatar: Set(true),
        ..Default::default()
    }).await?;

    Ok(WebResponse::default())
}

/// Delete avatar for the team.
pub async fn delete_team_avatar(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(game_id, operator.id).await?;
    let path = format!("games/{}/teams/{}/avatar", game_id, team.id);
    let _ = util::media::delete_img(path).await;

    let _ = cds_db::team::update::<Team>(cds_db::team::ActiveModel {
        id: Unchanged(team.id),
        has_avatar: Set(false),
        ..Default::default()
    }).await?;

    Ok(WebResponse::default())
}

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    response::IntoResponse,
};
use cds_db::{
    Team,
    sea_orm::{Set, Unchanged},
};
use cds_media::util::hash;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
    util,
    util::media::handle_multipart,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team_write_up))
        .route(
            "/",
            axum::routing::post(save_team_write_up)
                .layer(DefaultBodyLimit::max(50 * 1024 * 1024 /* MB */)),
        )
}

pub async fn get_team_write_up(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(game_id, operator.id).await?;

    util::media::get_write_up(game_id, team.id).await
}

/// Save a write-up for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn save_team_write_up(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = util::loader::prepare_game(game_id).await?;
    let team = util::loader::prepare_self_team(game.id, operator.id).await?;
    let path = format!("games/{}/teams/{}/writeup", game.id, team.id);

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    if now > game.ended_at || now < game.started_at {
        return Err(WebError::BadRequest(json!("game_is_not_ongoing")));
    }

    let data = handle_multipart(multipart, mime::PDF).await?;

    cds_media::delete_dir(path.clone()).await?;

    let filename = format!("{}.pdf", hash(data.clone()));

    let _ = cds_db::team::update::<Team>(cds_db::team::ActiveModel {
        id: Unchanged(team.id),
        has_write_up: Set(true),
        ..Default::default()
    })
    .await?;

    cds_media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse::default())
}

use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::{
    Team,
    sea_orm::{Set, Unchanged},
};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
    util,
    util::media::handle_multipart,
};

pub fn router() -> Router<Arc<AppState>> {
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
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let path = format!("games/{}/teams/{}", game_id, team.id);

    s.media.save(path, "avatar".to_owned(), data).await?;

    let _ = cds_db::team::update::<Team>(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            has_avatar: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}

/// Delete avatar for the team.
pub async fn delete_team_avatar(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    let path = format!("games/{}/teams/{}", game_id, team.id);

    s.media.delete(path, "avatar".to_owned()).await?;

    let _ = cds_db::team::update::<Team>(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            has_avatar: Set(false),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}

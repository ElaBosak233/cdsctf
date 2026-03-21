use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
    response::IntoResponse,
};
use cds_db::{
    Team,
    sea_orm::{Set, Unchanged},
};
use cds_media::util::hash;
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util,
    util::media::handle_multipart,
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_team_write_up).with_state(state.clone()))
        .routes(routes!(save_team_write_up).with_state(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Write-up file"),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_team_write_up(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    util::media::get_write_up(s.media.clone(), game_id, team.id).await
}

#[utoipa::path(
    post,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Write-up saved", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn save_team_write_up(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = util::loader::prepare_game(&s.db.conn, game_id).await?;
    let team = util::loader::prepare_self_team(&s.db.conn, game.id, operator.id).await?;
    let path = format!("games/{}/teams/{}/writeup", game.id, team.id);

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    if now > game.ended_at || now < game.started_at {
        return Err(WebError::BadRequest(json!("game_is_not_ongoing")));
    }

    let data = handle_multipart(multipart, mime::PDF).await?;

    s.media.delete_dir(path.clone()).await?;

    let filename = format!("{}.pdf", hash(data.clone()));

    let _ = cds_db::team::update::<Team>(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            has_writeup: Set(true),
            ..Default::default()
        },
    )
    .await?;

    s.media
        .save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(Json(EmptyJson::default()))
}

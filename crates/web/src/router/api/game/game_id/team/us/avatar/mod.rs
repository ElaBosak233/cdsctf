use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
};
use cds_db::{
    Team,
    sea_orm::{Set, Unchanged},
};
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
        .routes(routes!(save_team_avatar).with_state(state.clone()))
        .routes(routes!(delete_team_avatar).with_state(state.clone()))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Avatar saved", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn save_team_avatar(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
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

    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Avatar removed", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn delete_team_avatar(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
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

    Ok(Json(EmptyJson::default()))
}

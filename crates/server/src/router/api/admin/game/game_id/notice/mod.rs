use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{GameNotice, sea_orm::ActiveValue::Set};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Path},
    traits::{AppState, EmptySuccess, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::post(create_game_notice))
        .route("/{notice_id}", axum::routing::delete(delete_game_notice))
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(create_game_notice).with_state(state.clone()))
        .routes(routes!(delete_game_notice).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateGameNoticeRequest {
    pub game_id: Option<i64>,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameNoticeResponse {
    pub notice: GameNotice,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = CreateGameNoticeRequest,
    responses(
        (status = 200, description = "Notice created", body = GameNoticeResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn create_game_notice(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<CreateGameNoticeRequest>,
) -> Result<Json<GameNoticeResponse>, WebError> {
    let game_notice = cds_db::game_notice::create(
        &s.db.conn,
        cds_db::game_notice::ActiveModel {
            game_id: Set(game_id),
            title: Set(body.title),
            content: Set(body.content),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(GameNoticeResponse { notice: game_notice }))
}

#[utoipa::path(
    delete,
    path = "/{notice_id}",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("notice_id" = i64, Path, description = "Notice id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn delete_game_notice(
    State(s): State<Arc<AppState>>,
    Path((game_id, notice_id)): Path<(i64, i64)>,
) -> Result<Json<EmptySuccess>, WebError> {
    cds_db::game_notice::delete(&s.db.conn, notice_id, game_id).await?;
    Ok(Json(EmptySuccess::default()))
}

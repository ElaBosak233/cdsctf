mod challenge;
mod icon;
mod notice;
mod poster;
mod team;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::sea_orm::{
    ActiveValue::{Set, Unchanged},
    NotSet,
};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    router::api::game::game_id::GameDetailResponse,
    traits::{AppState, EmptySuccess, WebError},
};


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game).with_state(state.clone()))
        .routes(routes!(update_game).with_state(state.clone()))
        .routes(routes!(delete_game).with_state(state.clone()))
        .routes(routes!(calculate_game).with_state(state.clone()))
        .nest("/challenges", challenge::openapi_router(state.clone()))
        .nest("/teams", team::openapi_router(state.clone()))
        .nest("/notices", notice::openapi_router(state.clone()))
        .nest("/icon", icon::openapi_router(state.clone()))
        .nest("/poster", poster::openapi_router(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Game", body = GameDetailResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_game(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameDetailResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    Ok(Json(GameDetailResponse { game }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub writeup_required: Option<bool>,
    pub timeslots: Option<Vec<cds_db::game::Timeslot>>,
    pub started_at: Option<i64>,
    pub frozen_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[utoipa::path(
    put,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = UpdateGameRequest,
    responses(
        (status = 200, description = "Updated game", body = GameDetailResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn update_game(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    VJson(body): VJson<UpdateGameRequest>,
) -> Result<Json<GameDetailResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let game = cds_db::game::update(
        &s.db.conn,
        cds_db::game::ActiveModel {
            id: Unchanged(game.id),
            title: body.title.map_or(NotSet, Set),
            sketch: body.sketch.map_or(NotSet, |v| Set(Some(v))),
            description: body.description.map_or(NotSet, |v| Set(Some(v))),
            enabled: body.enabled.map_or(NotSet, Set),
            public: body.public.map_or(NotSet, Set),
            writeup_required: body.writeup_required.map_or(NotSet, Set),

            member_limit_min: body.member_limit_min.map_or(NotSet, Set),
            member_limit_max: body.member_limit_max.map_or(NotSet, Set),

            timeslots: body.timeslots.map_or(NotSet, Set),
            started_at: body.started_at.map_or(NotSet, Set),
            frozen_at: body.frozen_at.map_or(NotSet, Set),
            ended_at: body.ended_at.map_or(NotSet, Set),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(GameDetailResponse { game }))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn delete_game(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptySuccess>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let _ = cds_db::game::delete(&s.db.conn, game.id).await?;
    Ok(Json(EmptySuccess::default()))
}

#[utoipa::path(
    post,
    path = "/calculate",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Calculation queued", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn calculate_game(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptySuccess>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    s.queue
        .publish(
            "calculator",
            crate::worker::game_calculator::Payload {
                game_id: Some(game.id),
            },
        )
        .await?;

    Ok(Json(EmptySuccess::default()))
}

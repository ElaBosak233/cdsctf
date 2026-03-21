//! HTTP routing for `game` — Axum router wiring and OpenAPI route registration.

/// Defines the `game_id` submodule (see sibling `*.rs` files).
pub mod game_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{GameMini, game::FindGameOptions};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Query,
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game).with_state(state.clone()))
        .nest("/{game_id}", game_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GamesListResponse {
    pub items: Vec<GameMini>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(GetGameRequest),
    responses(
        (status = 200, description = "Games", body = GamesListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns game.
pub async fn get_game(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetGameRequest>,
) -> Result<Json<GamesListResponse>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(20);

    let (games, total) = cds_db::game::find(
        &s.db.conn,
        FindGameOptions {
            id: params.id,
            title: params.title,
            enabled: Some(true),
            page: Some(page),
            size: Some(size),
            sorts: params.sorts,
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(GamesListResponse {
        items: games,
        total,
    }))
}

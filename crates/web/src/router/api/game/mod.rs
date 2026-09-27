//! HTTP routing for `game` — Axum router wiring and OpenAPI route registration.

/// Defines the `game_id` submodule (see sibling `*.rs` files).
pub mod game_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{GameDetail, GameSummary, game::FindGameOptions, user::Group};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Query},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_games).with_state(state.clone()))
        .routes(routes!(crate::router::api::admin::game::create_game).with_state(state.clone()))
        .nest("/{game_id}", game_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListGamesRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GamesListResponse {
    pub games: Vec<GameSummary>,
    pub total: u64,
}

/// Lists enabled public games (collection). Use `GET /games/{game_id}` for one
/// game.
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(ListGamesRequest),
    responses(
        (status = 200, description = "Games", body = GamesListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_games"))]
pub async fn list_games(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Query(params): Query<ListGamesRequest>,
) -> Result<Json<serde_json::Value>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    if principal
        .operator
        .is_some_and(|user| user.group >= Group::Admin)
    {
        let (games, total) = cds_db::game::find::<GameDetail>(
            &s.db.conn,
            FindGameOptions {
                id: params.id,
                title: params.title,
                enabled: params.enabled,
                page: Some(page),
                size: Some(size),
                sorts: params.sorts,
                ..Default::default()
            },
        )
        .await?;
        return Ok(Json(serde_json::json!({ "games": games, "total": total })));
    }

    let (games, total) = cds_db::game::find::<GameSummary>(
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

    Ok(Json(serde_json::json!({ "games": games, "total": total })))
}

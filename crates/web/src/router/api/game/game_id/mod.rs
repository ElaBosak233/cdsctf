pub mod challenge;
mod icon;
mod notice;
mod poster;
pub mod team;

use std::{convert::Infallible, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    response::{
        IntoResponse, Sse,
        sse::{Event as SseEvent, KeepAlive},
    },
};
use cds_db::{
    Game, Submission, Team,
    team::{FindTeamOptions, State as TState},
};
use cds_event::SubscribeOptions;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Path, Query},
    traits::{AppState, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game).with_state(state.clone()))
        .routes(routes!(get_game_scoreboard).with_state(state.clone()))
        .routes(routes!(get_events).with_state(state.clone()))
        .nest("/challenges", challenge::router(state.clone()))
        .nest("/teams", team::router(state.clone()))
        .nest("/notices", notice::router(state.clone()))
        .nest("/icon", icon::router(state.clone()))
        .nest("/poster", poster::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameDetailResponse {
    pub game: Game,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Game", body = GameDetailResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_game(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameDetailResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    if !game.enabled {
        return Err(WebError::NotFound(json!("")));
    }

    Ok(Json(GameDetailResponse { game }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameScoreboardRequest {
    pub size: Option<u64>,
    pub page: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScoreRecord {
    pub team: Team,
    pub submissions: Vec<Submission>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameScoreboardResponse {
    pub items: Vec<ScoreRecord>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/scoreboard",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetGameScoreboardRequest,
    ),
    responses(
        (status = 200, description = "Scoreboard", body = GameScoreboardResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_game_scoreboard(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameScoreboardRequest>,
) -> Result<Json<GameScoreboardResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let (teams, total) = cds_db::team::find(
        &s.db.conn,
        FindTeamOptions {
            game_id: Some(game.id),
            state: Some(TState::Passed),
            sorts: Some("rank,-pts".to_string()),
            page: params.page,
            size: params.size,
            ..Default::default()
        },
    )
    .await?;

    let team_ids = teams.iter().map(|t: &Team| t.id).collect::<Vec<i64>>();

    let submissions = cds_db::submission::find_correct_by_team_ids_and_game_id::<Submission>(
        &s.db.conn, team_ids, game_id,
    )
    .await?;

    let mut result: Vec<ScoreRecord> = Vec::new();

    for team in teams {
        let submissions = submissions
            .iter()
            .filter(|s| s.team_id.is_some_and(|t| t == team.id))
            .cloned()
            .map(|submission| submission.desensitize())
            .collect::<Vec<Submission>>();

        result.push(ScoreRecord { team, submissions });
    }

    Ok(Json(GameScoreboardResponse {
        items: result,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetEventsRequest {
    pub token: String,
}

#[utoipa::path(
    get,
    path = "/events",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetEventsRequest,
    ),
    responses(
        (status = 200, description = "SSE stream", content_type = "text/event-stream"),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_events(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetEventsRequest>,
) -> Result<impl IntoResponse, WebError> {
    let stream = s
        .event
        .subscribe(SubscribeOptions {
            game_id: Some(game_id),
            token: Some(params.token),
        })
        .await?;

    let sse_stream = stream.map(|event| {
        let Ok(evt) = event;

        // SAFETY: Infallible.
        Ok::<SseEvent, Infallible>(SseEvent::default().json_data(evt).unwrap())
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}

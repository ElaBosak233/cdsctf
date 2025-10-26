mod challenge;
mod icon;
mod notice;
mod poster;
mod team;

use std::convert::Infallible;

use axum::{
    Router,
    response::{
        IntoResponse, Sse,
        sse::{Event as SseEvent, KeepAlive},
    },
};
use cds_db::{
    Game, Submission, Team,
    team::{FindTeamOptions, State},
};
use cds_event::SubscribeOptions;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Path, Query},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game))
        .nest("/challenges", challenge::router())
        .nest("/teams", team::router())
        .nest("/notices", notice::router())
        .nest("/icon", icon::router())
        .nest("/poster", poster::router())
        .route("/scoreboard", axum::routing::get(get_game_scoreboard))
        .route("/events", axum::routing::get(get_events))
}

pub async fn get_game(Path(game_id): Path<i64>) -> Result<WebResponse<Game>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    if !game.is_enabled {
        return Err(WebError::NotFound(json!("")));
    }

    Ok(WebResponse {
        data: Some(game),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameScoreboardRequest {
    pub size: Option<u64>,
    pub page: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub team: Team,
    pub submissions: Vec<Submission>,
}

pub async fn get_game_scoreboard(
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameScoreboardRequest>,
) -> Result<WebResponse<Vec<ScoreRecord>>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    let (teams, total) = cds_db::team::find(FindTeamOptions {
        game_id: Some(game.id),
        state: Some(State::Passed),
        sorts: Some("rank,-pts".to_string()),
        page: params.page,
        size: params.size,
        ..Default::default()
    })
    .await?;

    let team_ids = teams.iter().map(|t: &Team| t.id).collect::<Vec<i64>>();

    let submissions =
        cds_db::submission::find_correct_by_team_ids_and_game_id::<Submission>(team_ids, game_id)
            .await?;

    let mut result: Vec<ScoreRecord> = Vec::new();

    for team in teams {
        let submissions = submissions
            .iter()
            .filter(|s| s.team_id.is_some_and(|t| t == team.id))
            .cloned()
            .collect::<Vec<Submission>>();

        result.push(ScoreRecord { team, submissions });
    }

    Ok(WebResponse {
        data: Some(result),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetEventsRequest {
    pub token: String,
}

pub async fn get_events(
    Path(game_id): Path<i64>,
    Query(params): Query<GetEventsRequest>,
) -> Result<impl IntoResponse, WebError> {
    let stream = cds_event::subscribe(SubscribeOptions {
        game_id: Some(game_id),
        token: Some(params.token),
    })
    .await?;

    let sse_stream = stream.map(|event| {
        let Ok(evt) = event;
        Ok::<SseEvent, Infallible>(SseEvent::default().json_data(evt).unwrap())
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}

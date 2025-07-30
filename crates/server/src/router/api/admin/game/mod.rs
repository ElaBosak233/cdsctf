mod game_id;

use axum::{Router, http::StatusCode};
use cds_db::{
    Game,
    game::FindGameOptions,
    sea_orm::ActiveValue::{NotSet, Set},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    extract::{Query, VJson},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_games))
        .route("/", axum::routing::post(create_game))
        .nest("/{game_id}", game_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_games(
    Query(params): Query<GetGameRequest>,
) -> Result<WebResponse<Vec<Game>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (games, total) = cds_db::game::find::<Game>(FindGameOptions {
        id: params.id,
        title: params.title,
        is_enabled: params.is_enabled,
        page: Some(page),
        size: Some(size),
        sorts: params.sorts,
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(games),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateGameRequest {
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub is_need_write_up: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub timeslots: Option<Vec<cds_db::game::Timeslot>>,
    pub started_at: i64,
    pub ended_at: i64,
}

pub async fn create_game(
    VJson(body): VJson<CreateGameRequest>,
) -> Result<WebResponse<Game>, WebError> {
    let game = cds_db::game::create(cds_db::game::ActiveModel {
        title: Set(body.title),
        sketch: Set(body.sketch),
        description: Set(body.description),

        is_enabled: Set(body.is_enabled.unwrap_or(false)),
        is_public: Set(body.is_public.unwrap_or(false)),
        is_need_write_up: Set(body.is_need_write_up.unwrap_or(false)),

        member_limit_min: body.member_limit_min.map_or(NotSet, Set),
        member_limit_max: body.member_limit_max.map_or(NotSet, Set),

        timeslots: Set(body.timeslots.unwrap_or(vec![])),
        started_at: Set(body.started_at),
        ended_at: Set(body.ended_at),
        frozen_at: Set(body.ended_at),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game),
        ..Default::default()
    })
}

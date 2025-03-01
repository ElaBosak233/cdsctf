mod challenge;
mod icon;
mod notice;
mod poster;
mod team;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
    transfer::{GameTeam, Submission},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, EntityTrait, NotSet, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Path, Query, VJson},
    router::api::game::calculator,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_game))
        .route("/", axum::routing::delete(delete_game))
        .nest("/challenges", challenge::router())
        .nest("/teams", team::router())
        .nest("/notices", notice::router())
        .nest("/icon", icon::router())
        .nest("/poster", poster::router())
        .route("/calculate", axum::routing::post(calculate_game))
        .route("/scoreboard", axum::routing::get(get_game_scoreboard))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub timeslots: Option<Vec<cds_db::entity::game::Timeslot>>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub frozen_at: Option<i64>,
}

pub async fn update_game(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
    VJson(mut body): VJson<UpdateGameRequest>,
) -> Result<WebResponse<cds_db::transfer::Game>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let game = cds_db::entity::game::ActiveModel {
        id: Unchanged(game.id),
        title: body.title.map_or(NotSet, Set),
        sketch: body.sketch.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        is_public: body.is_public.map_or(NotSet, Set),
        is_need_write_up: body.is_need_write_up.map_or(NotSet, Set),

        member_limit_min: body.member_limit_min.map_or(NotSet, Set),
        member_limit_max: body.member_limit_max.map_or(NotSet, Set),

        timeslots: body.timeslots.map_or(NotSet, Set),
        started_at: body.started_at.map_or(NotSet, Set),
        ended_at: body.ended_at.map_or(NotSet, Set),
        frozen_at: body.frozen_at.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game = cds_db::transfer::Game::from(game);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game),
        ..Default::default()
    })
}

pub async fn delete_game(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let _ = cds_db::entity::game::Entity::delete_by_id(game.id)
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn calculate_game(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    cds_queue::publish("calculator", calculator::Payload {
        game_id: Some(game_id),
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
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
    pub game_team: GameTeam,
    pub submissions: Vec<Submission>,
}

pub async fn get_game_scoreboard(
    Path(game_id): Path<i64>, Query(params): Query<GetGameScoreboardRequest>,
) -> Result<WebResponse<Vec<ScoreRecord>>, WebError> {
    let mut sql = cds_db::entity::game_team::Entity::find()
        .filter(cds_db::entity::game_team::Column::GameId.eq(game_id))
        .order_by(cds_db::entity::game_team::Column::Pts, Order::Desc);

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let game_teams = sql.all(get_db()).await?;
    let mut game_teams = game_teams
        .into_iter()
        .map(GameTeam::from)
        .collect::<Vec<GameTeam>>();

    game_teams = cds_db::transfer::game_team::preload(game_teams).await?;

    let team_ids = game_teams.iter().map(|t| t.team_id).collect::<Vec<i64>>();

    let submissions = cds_db::transfer::submission::get_by_game_id_and_team_ids(
        game_id,
        team_ids,
        Some(Status::Correct),
    )
    .await?;

    let mut result: Vec<ScoreRecord> = Vec::new();

    for game_team in game_teams {
        let mut submissions = submissions
            .iter()
            .filter(|s| s.team_id.unwrap() == game_team.team_id)
            .cloned()
            .collect::<Vec<Submission>>();
        for submission in submissions.iter_mut() {
            submission.desensitize();
            submission.team = None;
            submission.game = None;
        }

        result.push(ScoreRecord {
            game_team,
            submissions,
        });
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(result),
        total: Some(total),
        ..Default::default()
    })
}

mod challenge;
mod icon;
mod notice;
mod poster;
mod team;

use axum::{Router, http::StatusCode};
use cds_db::{
    get_db,
    sea_orm::{
        ActiveModelTrait,
        ActiveValue::{Set, Unchanged},
        EntityTrait, NotSet,
    },
    transfer::Game,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
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
    pub frozen_at: Option<i64>,
    pub ended_at: Option<i64>,
}

pub async fn update_game(
    Path(game_id): Path<i64>,
    VJson(body): VJson<UpdateGameRequest>,
) -> Result<WebResponse<Game>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

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
        frozen_at: body.frozen_at.map_or(NotSet, Set),
        ended_at: body.ended_at.map_or(NotSet, Set),
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

pub async fn delete_game(Path(game_id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    let _ = cds_db::entity::game::Entity::delete_by_id(game.id)
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn calculate_game(Path(game_id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    cds_queue::publish("calculator", crate::worker::game_calculator::Payload {
        game_id: Some(game.id),
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

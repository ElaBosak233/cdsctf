pub mod calculator;
mod game_id;

use std::str::FromStr;

use axum::{Router, http::StatusCode, response::IntoResponse};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Query, VJson},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    calculator::init().await;

    Router::new()
        .route("/", axum::routing::get(get_game))
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

/// Get games with given params.
pub async fn get_game(
    Extension(ext): Extension<Ext>, Query(params): Query<GetGameRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Game>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && !params.is_enabled.unwrap_or(false) {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut sql = cds_db::entity::game::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::game::Column::Id.eq(id));
    }

    if let Some(title) = params.title {
        sql = sql.filter(cds_db::entity::game::Column::Title.contains(title));
    }

    if let Some(is_enabled) = params.is_enabled {
        sql = sql.filter(cds_db::entity::game::Column::IsEnabled.eq(is_enabled));
    }

    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match cds_db::entity::game::Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let games = sql
        .all(get_db())
        .await?
        .into_iter()
        .map(cds_db::transfer::Game::from)
        .collect::<Vec<cds_db::transfer::Game>>();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
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
    pub timeslots: Option<Vec<cds_db::entity::game::Timeslot>>,
    pub started_at: i64,
    pub ended_at: i64,
}

pub async fn create_game(
    Extension(ext): Extension<Ext>, VJson(body): VJson<CreateGameRequest>,
) -> Result<WebResponse<cds_db::transfer::Game>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::ActiveModel {
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
    }
    .insert(get_db())
    .await?;
    let game = cds_db::transfer::Game::from(game);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..Default::default()
    })
}

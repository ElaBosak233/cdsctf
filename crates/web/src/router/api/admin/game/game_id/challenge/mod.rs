use axum::{Router, http::StatusCode};
use cds_db::{
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, NotSet, PaginatorTrait,
        QueryFilter, QuerySelect,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Path, Query},
    model::game_challenge::GameChallenge,
    traits::{Ext, WebError, WebResponse},
};

mod challenge_id;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game_challenge))
        .route("/", axum::routing::post(create_game_challenge))
        .nest("/{challenge_id}", challenge_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
    pub category: Option<i32>,

    pub page: Option<u64>,
    pub size: Option<u64>,
}

/// Get challenges by given params.
pub async fn get_game_challenge(
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<WebResponse<Vec<GameChallenge>>, WebError> {
    // Using inner join to access fields in related tables.
    let mut sql = cds_db::entity::game_challenge::Entity::base_find();

    sql = sql.filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id));

    if let Some(challenge_id) = params.challenge_id {
        sql = sql.filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(category) = params.category {
        sql = sql.filter(cds_db::entity::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let game_challenges = sql.into_model::<GameChallenge>().all(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_challenges),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameChallengeRequest {
    pub challenge_id: Uuid,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub bonus_ratios: Option<Vec<i64>>,
    pub frozen_at: Option<Option<i64>>,
}

pub async fn create_game_challenge(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Json(body): Json<CreateGameChallengeRequest>,
) -> Result<WebResponse<GameChallenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = crate::util::loader::prepare_game(game_id).await?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let is_challenge_in_game = cds_db::entity::game_challenge::Entity::find()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game.id))
        .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge.id))
        .count(get_db())
        .await?
        > 0;

    if is_challenge_in_game {
        return Err(WebError::Conflict(json!("challenge_already_in_game")));
    }

    let game_challenge = cds_db::entity::game_challenge::ActiveModel {
        game_id: Set(game.id),
        challenge_id: Set(challenge.id),
        difficulty: body.difficulty.map_or(NotSet, Set),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        max_pts: body.max_pts.map_or(NotSet, Set),
        min_pts: body.min_pts.map_or(NotSet, Set),
        bonus_ratios: body.bonus_ratios.map_or(Set(vec![5, 3, 1]), Set),
        frozen_at: body.frozen_at.map_or(NotSet, Set),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    cds_queue::publish(
        "calculator",
        crate::worker::game_calculator::Payload {
            game_id: Some(game.id),
        },
    )
    .await?;

    let game_challenge = cds_db::entity::game_challenge::Entity::base_find()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_challenge.game_id))
        .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(game_challenge.challenge_id))
        .into_model::<GameChallenge>()
        .one(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: game_challenge,
        ..Default::default()
    })
}

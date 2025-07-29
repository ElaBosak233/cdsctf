use axum::Router;
use cds_db::{
    GameChallenge,
    game_challenge::FindGameChallengeOptions,
    sea_orm::{ActiveValue::Set, NotSet},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Json, Path, Query},
    traits::{WebError, WebResponse},
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
}

/// Get challenges by given params.
pub async fn get_game_challenge(
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<WebResponse<Vec<GameChallenge>>, WebError> {
    let (game_challenges, _) = cds_db::game_challenge::find(FindGameChallengeOptions {
        game_id: Some(game_id),
        challenge_id: params.challenge_id,
        category: params.category,
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        data: Some(game_challenges),
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
    Path(game_id): Path<i64>,
    Json(body): Json<CreateGameChallengeRequest>,
) -> Result<WebResponse<GameChallenge>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    let challenge = crate::util::loader::prepare_challenge(body.challenge_id).await?;

    if cds_db::util::is_challenge_in_game(challenge.id, game.id).await? {
        return Err(WebError::Conflict(json!("challenge_already_in_game")));
    }

    let game_challenge =
        cds_db::game_challenge::create::<GameChallenge>(cds_db::game_challenge::ActiveModel {
            game_id: Set(game.id),
            challenge_id: Set(challenge.id),
            difficulty: body.difficulty.map_or(NotSet, Set),
            is_enabled: body.is_enabled.map_or(Set(false), Set),
            max_pts: body.max_pts.map_or(NotSet, Set),
            min_pts: body.min_pts.map_or(NotSet, Set),
            bonus_ratios: body.bonus_ratios.map_or(Set(vec![]), Set),
            frozen_at: body.frozen_at.map_or(NotSet, Set),
            ..Default::default()
        })
        .await?;

    cds_queue::publish(
        "calculator",
        crate::worker::game_calculator::Payload {
            game_id: Some(game.id),
        },
    )
    .await?;

    Ok(WebResponse {
        data: Some(game_challenge),
        ..Default::default()
    })
}

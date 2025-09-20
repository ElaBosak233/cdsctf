use axum::{Router, http::StatusCode};
use cds_db::{
    GameChallenge,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    user::Group,
};
use cds_event::types::game_challenge::{GameChallengeEvent, GameChallengeEventType};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::serde_as;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_game_challenge))
        .route("/", axum::routing::delete(delete_game_challenge))
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGameChallengeRequest {
    pub challenge_id: Option<Uuid>,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub bonus_ratios: Option<Vec<i64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub frozen_at: Option<Option<i64>>,
}

pub async fn update_game_challenge(
    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, challenge_id)): Path<(i64, i64)>,
    Json(body): Json<UpdateGameChallengeRequest>,
) -> Result<WebResponse<GameChallenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = crate::util::loader::prepare_game_challenge(game_id, challenge_id).await?;

    let new_game_challenge =
        cds_db::game_challenge::update::<GameChallenge>(cds_db::game_challenge::ActiveModel {
            game_id: Unchanged(game_challenge.game_id),
            challenge_id: Unchanged(game_challenge.challenge_id),
            is_enabled: body.is_enabled.map_or(NotSet, Set),
            difficulty: body.difficulty.map_or(NotSet, Set),
            max_pts: body.max_pts.map_or(NotSet, Set),
            min_pts: body.min_pts.map_or(NotSet, Set),
            bonus_ratios: body.bonus_ratios.map_or(NotSet, Set),
            frozen_at: body.frozen_at.map_or(NotSet, Set),
            ..Default::default()
        })
        .await?;

    cds_queue::publish(
        "calculator",
        crate::worker::game_calculator::Payload {
            game_id: Some(new_game_challenge.game_id),
        },
    )
    .await?;

    if new_game_challenge.is_enabled != game_challenge.is_enabled {
        cds_event::push(cds_event::types::Event::GameChallenge(GameChallengeEvent {
            type_: if new_game_challenge.is_enabled {
                GameChallengeEventType::Up
            } else {
                GameChallengeEventType::Down
            },
        }))
        .await?;
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(new_game_challenge),
        ..Default::default()
    })
}

pub async fn delete_game_challenge(
    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, challenge_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = crate::util::loader::prepare_game_challenge(game_id, challenge_id).await?;

    cds_db::game_challenge::delete(game_challenge.game_id, game_challenge.challenge_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

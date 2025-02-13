use axum::{Router, http::StatusCode};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, Condition, EntityTrait, NotSet, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::serde_as;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Path},
    traits::{Ext, WebError, WebResponse},
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
    Extension(ext): Extension<Ext>, Path((game_id, challenge_id)): Path<(i64, Uuid)>,
    Json(mut body): Json<UpdateGameChallengeRequest>,
) -> Result<WebResponse<cds_db::transfer::GameChallenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = cds_db::entity::game_challenge::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_challenge_not_found")))?;

    let game_challenge = cds_db::entity::game_challenge::ActiveModel {
        game_id: Unchanged(game_challenge.game_id),
        challenge_id: Unchanged(game_challenge.challenge_id),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        difficulty: body.difficulty.map_or(NotSet, Set),
        max_pts: body.max_pts.map_or(NotSet, Set),
        min_pts: body.min_pts.map_or(NotSet, Set),
        bonus_ratios: body.bonus_ratios.map_or(NotSet, Set),
        frozen_at: body.frozen_at.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game_challenge = cds_db::transfer::GameChallenge::from(game_challenge);

    cds_queue::publish(
        "calculator",
        crate::router::api::game::calculator::Payload {
            game_id: Some(game_challenge.game_id),
        },
    )
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..Default::default()
    })
}

pub async fn delete_game_challenge(
    Extension(ext): Extension<Ext>, Path((game_id, challenge_id)): Path<(i64, Uuid)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = cds_db::entity::game_challenge::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_challenge_not_found")))?;

    let _ = cds_db::entity::game_challenge::Entity::delete_many()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_challenge.game_id))
        .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(game_challenge.challenge_id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

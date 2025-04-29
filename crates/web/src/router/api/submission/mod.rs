use std::str::FromStr;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{submission::Status, team::State},
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, Order,
        PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    model::submission::Submission,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_submission))
        .route("/", axum::routing::post(create_submission))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_submission(
    Extension(ext): Extension<Ext>,
    Query(params): Query<GetSubmissionRequest>,
) -> Result<WebResponse<Vec<Submission>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(1000);

    let mut sql = cds_db::entity::submission::Entity::base_find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::submission::Column::Id.eq(id));
    }

    if let Some(user_id) = params.user_id {
        sql = sql.filter(cds_db::entity::submission::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = params.team_id {
        sql = sql.filter(cds_db::entity::submission::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = params.game_id {
        sql = sql.filter(cds_db::entity::submission::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = params.challenge_id {
        sql = sql.filter(cds_db::entity::submission::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = params.status {
        sql = sql.filter(cds_db::entity::submission::Column::Status.eq(status));
    }

    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match cds_db::entity::submission::Column::from_str(
                sort.replace("-", "").as_str(),
            ) {
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

    let offset = (page - 1) * size;
    sql = sql.offset(offset).limit(size);

    let submissions = sql.into_model::<Submission>().all(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(submissions),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSubmissionRequest {
    pub content: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: uuid::Uuid,
}

pub async fn create_submission(
    Extension(ext): Extension<Ext>,
    Json(mut body): Json<CreateSubmissionRequest>,
) -> Result<WebResponse<Submission>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    body.user_id = Some(operator.id);

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    if body.game_id.is_some() != body.team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    // If the submission is not in game mode, challenge must be public.
    if !challenge.is_public && (body.game_id.is_none() || body.team_id.is_none()) {
        return Err(WebError::BadRequest(json!("challenge_not_found")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let game = cds_db::entity::game::Entity::find_by_id(game_id)
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_not_found")))?;

        let _ = cds_db::entity::game_challenge::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::game_challenge::Column::GameId.eq(game.id))
                    .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(body.challenge_id)),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_challenge_not_found")));

        let _ = cds_db::entity::team::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::team::Column::Id.eq(team_id))
                    .add(cds_db::entity::team::Column::GameId.eq(game.id))
                    .add(cds_db::entity::team::Column::State.eq(State::Passed)),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")));
    }

    let submission = cds_db::entity::submission::ActiveModel {
        content: Set(body.content),
        user_id: body.user_id.map_or(NotSet, Set),
        team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
        game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
        challenge_id: Set(body.challenge_id),
        status: Set(Status::Pending),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    cds_queue::publish("checker", submission.id).await?;

    let submission = cds_db::entity::submission::Entity::base_find()
        .filter(cds_db::entity::submission::Column::Id.eq(submission.id))
        .into_model::<Submission>()
        .one(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: submission,
        ..Default::default()
    })
}

pub mod checker;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
    transfer::Submission,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    checker::init().await;

    Router::new()
        .route("/", axum::routing::get(get))
        .route("/{id}", axum::routing::get(get_by_id))
        .route("/", axum::routing::post(create))
        .route("/{id}", axum::routing::delete(delete))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<Submission>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut sql = cds_db::entity::submission::Entity::find();

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

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let submissions = sql.all(get_db()).await?;
    let mut submissions = submissions
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();

    submissions = cds_db::transfer::submission::preload(submissions).await?;

    match params.is_detailed {
        Some(true) => {
            for submission in submissions.iter_mut() {
                submission.desensitize();
            }
        }
        _ => {}
    }

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submissions),
        total: Some(total),
        ..WebResponse::default()
    })
}

pub async fn get_by_id(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<Submission>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let submission = cds_db::entity::submission::Entity::find_by_id(id)
        .one(get_db())
        .await?;

    if submission.is_none() {
        return Err(WebError::NotFound(json!("")));
    }

    let submission = submission.unwrap();
    let mut submission = cds_db::transfer::Submission::from(submission);
    submission.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub flag: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: uuid::Uuid,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreateRequest>,
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

    if !challenge.is_public && (body.game_id.is_none() || body.team_id.is_none()) {
        return Err(WebError::BadRequest(json!("challenge_not_found")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let game = cds_db::entity::game::Entity::find_by_id(game_id)
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_not_found")))?;

        let team = cds_db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?;

        let _ = cds_db::entity::game_challenge::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::game_challenge::Column::GameId.eq(game.id))
                    .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(body.challenge_id)),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_challenge_not_found")));

        let _ = cds_db::entity::game_challenge::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::game_team::Column::TeamId.eq(team.id))
                    .add(cds_db::entity::game_team::Column::GameId.eq(game.id))
                    .add(cds_db::entity::game_team::Column::IsAllowed.eq(true)),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_team_not_found")));
    }

    let submission = cds_db::entity::submission::ActiveModel {
        flag: Set(body.flag),
        user_id: body.user_id.map_or(NotSet, Set),
        team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
        game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
        challenge_id: Set(body.challenge_id),
        status: Set(Status::Pending),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let submission = cds_db::transfer::Submission::from(submission);

    cds_queue::publish("checker", submission.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..WebResponse::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::submission::Entity::delete_by_id(id)
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

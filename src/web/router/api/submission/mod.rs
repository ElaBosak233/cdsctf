pub mod checker;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};
use serde::{Deserialize, Serialize};

pub async fn router() -> Router {
    checker::init().await;

    Router::new()
        .route("/", axum::routing::get(get))
        .route("/:id", axum::routing::get(get_by_id))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::delete(delete))
}

use crate::{
    db::{
        entity::{submission::Status, user::Group},
        get_db,
    },
    web::traits::{Ext, WebError, WebResult},
};

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
) -> Result<WebResult<Vec<crate::shared::Submission>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(String::new()));
    }

    let (mut submissions, total) = crate::shared::submission::find(
        params.id,
        params.user_id,
        params.team_id,
        params.game_id,
        params.challenge_id,
        params.status,
        params.page,
        params.size,
    )
    .await?;

    let is_detailed = params.is_detailed.unwrap_or(false);
    for submission in submissions.iter_mut() {
        if !is_detailed {
            submission.desensitize();
        }
    }

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(submissions),
        total: Some(total),
        ..WebResult::default()
    })
}

pub async fn get_by_id(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<crate::shared::Submission>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let submission = crate::db::entity::submission::Entity::find_by_id(id)
        .one(get_db())
        .await?;

    if submission.is_none() {
        return Err(WebError::NotFound(String::from("")));
    }

    let submission = submission.unwrap();
    let mut submission = crate::shared::Submission::from(submission);
    submission.desensitize();

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub flag: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreateRequest>,
) -> Result<WebResult<crate::shared::Submission>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    body.user_id = Some(operator.id);

    if let Some(challenge_id) = body.challenge_id {
        let challenge = crate::db::entity::challenge::Entity::find_by_id(challenge_id)
            .one(get_db())
            .await?;

        if challenge.is_none() {
            return Err(WebError::BadRequest(String::from("challenge_not_found")));
        }
    }

    if let Some(game_id) = body.game_id {
        let game = crate::db::entity::game::Entity::find_by_id(game_id)
            .one(get_db())
            .await?;

        if game.is_none() {
            return Err(WebError::BadRequest(String::from("game_not_found")));
        }
    }

    if let Some(team_id) = body.team_id {
        let team = crate::db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?;

        if team.is_none() {
            return Err(WebError::BadRequest(String::from("team_not_found")));
        }
    }

    let submission = crate::db::entity::submission::ActiveModel {
        flag: Set(body.flag),
        user_id: body.user_id.map_or(NotSet, |v| Set(v)),
        team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
        game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
        challenge_id: body.challenge_id.map_or(NotSet, |v| Set(v)),
        status: Set(Status::Pending),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let submission = crate::shared::Submission::from(submission);

    crate::queue::publish("checker", submission.id).await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..WebResult::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::db::entity::submission::Entity::delete_by_id(id)
        .exec(get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};

use crate::{database::get_db, web::traits::Ext};
use crate::{
    model::submission::Status,
    web::{model::submission::*, traits::WebError},
};

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(String::new()));
    }

    let (mut submissions, total) = crate::model::submission::find(
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
            submission.simplify();
        }
    }

    return Ok((
        StatusCode::OK,
        Json(GetResponse {
            code: StatusCode::OK.as_u16(),
            data: submissions,
            total: total,
        }),
    ));
}

pub async fn get_by_id(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let submission = crate::model::submission::Entity::find_by_id(id)
        .one(&get_db())
        .await?;

    if submission.is_none() {
        return Err(WebError::NotFound(String::from("")));
    }

    let mut submission = submission.unwrap();
    submission.simplify();

    return Ok((
        StatusCode::OK,
        Json(GetByIDResponse {
            code: StatusCode::OK.as_u16(),
            data: submission,
        }),
    ));
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.unwrap();
    body.user_id = Some(operator.id);

    if let Some(challenge_id) = body.challenge_id {
        let challenge = crate::model::challenge::Entity::find_by_id(challenge_id)
            .one(&get_db())
            .await
            .unwrap();

        if challenge.is_none() {
            return Err(WebError::BadRequest(String::from("challenge_not_found")));
        }
    }

    if let Some(game_id) = body.game_id {
        let game = crate::model::game::Entity::find_by_id(game_id)
            .one(&get_db())
            .await
            .unwrap();

        if game.is_none() {
            return Err(WebError::BadRequest(String::from("game_not_found")));
        }
    }

    if let Some(team_id) = body.team_id {
        let team = crate::model::team::Entity::find_by_id(team_id)
            .one(&get_db())
            .await
            .unwrap();

        if team.is_none() {
            return Err(WebError::BadRequest(String::from("team_not_found")));
        }
    }

    let submission = crate::model::submission::ActiveModel {
        flag: Set(body.flag),
        user_id: body.user_id.map_or(NotSet, |v| Set(v)),
        team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
        game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
        challenge_id: body.challenge_id.map_or(NotSet, |v| Set(v)),
        status: Set(Status::Pending),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    crate::queue::publish("checker", submission.id).await?;

    return Ok((
        StatusCode::OK,
        Json(CreateResponse {
            code: StatusCode::OK.as_u16(),
            data: submission,
        }),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::submission::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    return Ok((
        StatusCode::OK,
        Json(DeleteResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

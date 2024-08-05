use anyhow::anyhow;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde_json::json;

use crate::{
    checker,
    web::{service::submission as submission_service, traits::Error},
};
use crate::{database::get_db, web::traits::Ext};

pub async fn get(
    Extension(ext): Extension<Ext>,
    Query(params): Query<crate::model::submission::request::FindRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && params.is_detailed.unwrap_or(false) {
        return Err(Error::Forbidden(String::new()));
    }

    let result = submission_service::find(params).await;

    if let Err(err) = result {
        return Err(Error::OtherError(anyhow!("{:?}", err)));
    }

    let (submissions, total) = result.unwrap();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(submissions),
            "total": total,
        })),
    ));
}

pub async fn get_by_id(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let submission = crate::model::submission::Entity::find_by_id(id)
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    if submission.is_none() {
        return Err(Error::NotFound(String::from("")));
    }

    let mut submission = submission.unwrap();
    submission.simplify();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(submission),
        })),
    ));
}

pub async fn create(
    Extension(ext): Extension<Ext>,
    Json(mut body): Json<crate::model::submission::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    body.user_id = Some(operator.id);

    if let Some(challenge_id) = body.challenge_id {
        let challenge = crate::model::challenge::Entity::find_by_id(challenge_id)
            .one(&get_db())
            .await
            .unwrap();

        if challenge.is_none() {
            return Err(Error::BadRequest(String::from("challenge_not_found")));
        }
    }

    if let Some(game_id) = body.game_id {
        let game = crate::model::game::Entity::find_by_id(game_id)
            .one(&get_db())
            .await
            .unwrap();

        if game.is_none() {
            return Err(Error::BadRequest(String::from("game_not_found")));
        }
    }

    if let Some(team_id) = body.team_id {
        let team = crate::model::team::Entity::find_by_id(team_id)
            .one(&get_db())
            .await
            .unwrap();

        if team.is_none() {
            return Err(Error::BadRequest(String::from("team_not_found")));
        }
    }

    let submission = crate::model::submission::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    checker::get_tx().send(submission.id.clone()).unwrap();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(submission),
        })),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let _ = crate::model::submission::Entity::delete_by_id(id)
        .exec(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

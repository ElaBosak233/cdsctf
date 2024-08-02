use anyhow::anyhow;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

use crate::web::{service, traits::Error};
use crate::{database::get_db, web::traits::Ext};

pub async fn get(
    Query(params): Query<crate::model::pod::request::FindRequest>,
) -> Result<impl IntoResponse, Error> {
    let (mut pods, total) = crate::model::pod::find(
        params.id,
        params.name,
        params.user_id,
        params.team_id,
        params.game_id,
        params.challenge_id,
        params.is_available,
    )
    .await
    .map_err(|err| Error::DatabaseError(err))?;

    if let Some(is_detailed) = params.is_detailed {
        if !is_detailed {
            for pod in pods.iter_mut() {
                pod.flag = None;
            }
        }
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(pods),
            "total": total,
        })),
    ));
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<crate::model::pod::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.clone().unwrap();
    body.user_id = Some(operator.id);

    let result = service::pod::create(body).await;

    if let Err(err) = result {
        return Err(Error::OtherError(anyhow!("{:?}", err)));
    }

    let pod = result.unwrap();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(pod),
        })),
    ));
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.clone().unwrap();

    let pod = crate::model::pod::Entity::find()
        .filter(crate::model::pod::Column::Id.eq(id))
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::NotFound(String::new()))?;

    if !(operator.group == "admin"
        || operator.id == pod.user_id
        || operator
            .teams
            .iter()
            .any(|team| Some(team.id) == pod.team_id))
    {
        return Err(Error::Forbidden(String::new()));
    }

    let result = service::pod::update(id).await;

    if let Err(err) = result {
        return Err(Error::OtherError(anyhow!("{:?}", err)));
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.clone().unwrap();
    let pod = crate::model::pod::Entity::find()
        .filter(crate::model::pod::Column::Id.eq(id))
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::NotFound(String::new()))?;

    if !(operator.group == "admin"
        || operator.id == pod.user_id
        || operator
            .teams
            .iter()
            .any(|team| Some(team.id) == pod.team_id))
    {
        return Err(Error::Forbidden(String::new()));
    }

    let result = service::pod::delete(id).await;

    if let Err(err) = result {
        return Err(Error::OtherError(anyhow!("{:?}", err)));
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

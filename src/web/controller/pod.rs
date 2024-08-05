use anyhow::anyhow;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use regex::Regex;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::web::traits::Error;
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

    let challenge = crate::model::challenge::Entity::find_by_id(body.challenge_id)
        .one(&get_db())
        .await?;

    let challenge = challenge.unwrap();

    let ctn_name = format!("cds-{}", Uuid::new_v4().simple().to_string());

    if challenge.flags.clone().into_iter().next().is_none() {
        return Err(Error::BadRequest(String::from("no_flag")));
    }

    let mut injected_flag = challenge.flags.clone().into_iter().next().unwrap();

    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])\]").unwrap();
    if injected_flag.type_ == crate::model::challenge::flag::Type::Dynamic {
        injected_flag.value = re
            .replace_all(
                &injected_flag.value,
                uuid::Uuid::new_v4().simple().to_string(),
            )
            .to_string();
    }

    let nats = crate::container::get_container()
        .await
        .create(ctn_name.clone(), challenge.clone(), injected_flag.clone())
        .await
        .map_err(|err| Error::OtherError(anyhow!("{:?}", err)))?;

    let mut pod = crate::model::pod::ActiveModel {
        name: Set(ctn_name),
        user_id: Set(body.user_id.clone().unwrap()),
        team_id: Set(body.team_id.clone()),
        game_id: Set(body.game_id.clone()),
        challenge_id: Set(body.challenge_id.clone()),
        flag: Set(Some(injected_flag.value)),
        removed_at: Set(chrono::Utc::now().timestamp() + challenge.duration),
        nats: Set(nats),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    pod.simplify();

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

    let challenge = crate::model::challenge::Entity::find_by_id(pod.challenge_id)
        .one(&get_db())
        .await?;
    let challenge = challenge.unwrap();

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp() + challenge.duration);
    let _ = pod.update(&get_db()).await;

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
    let pod = crate::model::pod::Entity::find_by_id(id)
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

    crate::container::get_container()
        .await
        .delete(pod.name.clone())
        .await;

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp());

    let _ = pod.update(&get_db()).await?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

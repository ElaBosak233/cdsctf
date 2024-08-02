use anyhow::anyhow;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::web::{service::submission as submission_service, traits::Error};
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

pub async fn create(
    Extension(ext): Extension<Ext>,
    Json(mut body): Json<crate::model::submission::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    body.user_id = Some(operator.id);

    let result = submission_service::create(body).await;

    if let Err(err) = result {
        return Err(Error::OtherError(anyhow!("{:?}", err)));
    }

    let submission = result.unwrap();

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

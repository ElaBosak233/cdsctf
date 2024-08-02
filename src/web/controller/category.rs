use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::model::category::request::{CreateRequest, FindRequest, UpdateRequest};
use crate::util::validate;
use crate::web::traits::Error;

pub async fn get(Query(params): Query<FindRequest>) -> Result<impl IntoResponse, Error> {
    let (categories, total) = crate::model::category::find(params.id, params.name)
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(categories),
            "total": total,
        })),
    ));
}

pub async fn create(
    validate::Json(body): validate::Json<CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let category = crate::model::category::create(body.into())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(category),
        })),
    ));
}

pub async fn update(
    Path(id): Path<i64>, validate::Json(mut body): validate::Json<UpdateRequest>,
) -> Result<impl IntoResponse, Error> {
    body.id = Some(id);
    let category = crate::model::category::create(body.into())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": category,
        })),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let _ = crate::model::category::delete(id)
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16()
        })),
    ));
}

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Multipart, Path, Query},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::web::traits::Ext;
use crate::{database::get_db, web::service};
use crate::{util::validate, web::traits::Error};

pub async fn get(
    Extension(ext): Extension<Ext>,
    Query(params): Query<crate::model::challenge::request::FindRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && params.is_detailed.unwrap_or(false) {
        return Err(Error::Forbidden(String::new()));
    }

    let (mut challenges, total) = crate::model::challenge::find(
        params.id,
        params.title,
        params.category_id,
        params.is_practicable,
        params.is_dynamic,
        params.page,
        params.size,
    )
    .await
    .map_err(|err| Error::DatabaseError(err))?;

    for challenge in challenges.iter_mut() {
        let is_detailed = params.is_detailed.unwrap_or(false);
        if !is_detailed {
            challenge.flags.clear();
        }
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenges),
            "total": total,
        })),
    ));
}

pub async fn get_status(
    Json(body): Json<crate::model::challenge::request::StatusRequest>,
) -> Result<impl IntoResponse, Error> {
    let result = service::challenge::status(body).await;

    if let Err(err) = result {
        return Err(Error::OtherError(anyhow!("{:?}", err)));
    }

    let status = result.unwrap();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(status),
        })),
    ));
}

pub async fn create(
    Json(body): Json<crate::model::challenge::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let challenge = crate::model::challenge::create(body.into())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenge),
        })),
    ));
}

pub async fn update(
    Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<crate::model::challenge::request::UpdateRequest>,
) -> Result<impl IntoResponse, Error> {
    body.id = Some(id);

    let challenge = crate::model::challenge::update(body.into())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenge),
        })),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let _ = crate::model::challenge::Entity::delete_by_id(id)
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

pub async fn get_attachment(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(buffer))
                .unwrap());
        }
        None => return Err(Error::NotFound(String::new())),
    }
}

pub async fn get_attachment_metadata(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return Ok((
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": {
                        "filename": filename,
                        "size": size,
                    },
                })),
            ));
        }
        None => return Err(Error::NotFound(String::new())),
    }
}

pub async fn save_attachment(
    Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, Error> {
    let path = format!("challenges/{}/attachment", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(Error::BadRequest(String::from("size_too_large")));
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    let _ = crate::media::save(path, filename, data)
        .await
        .map_err(|_| Error::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete_attachment(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("challenges/{}/attachment", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| Error::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

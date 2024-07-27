use axum::{
    extract::{Multipart, Path, Query},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use crate::{server::service, traits::Ext};

use crate::util::validate;

/// **Find** can be used to find challenges.
///
/// ## Arguments
/// - `params`: A `FindRequest` struct containing the parameters for the find operation.
/// - `ext`: An `Ext` struct containing the current operator.
///
/// ## Returns
/// - `200`: find successfully.
/// - `403`: operator does not have permission to find challenges.
/// - `400`: find failed.
pub async fn find(
    Extension(ext): Extension<Ext>,
    Query(params): Query<crate::model::challenge::request::FindRequest>,
) -> impl IntoResponse {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && params.is_detailed.unwrap_or(false) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
            })),
        );
    }

    match service::challenge::find(params).await {
        Ok((challenges, total)) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(challenges),
                "total": total,
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
            })),
        ),
    }
}

pub async fn status(
    Json(body): Json<crate::model::challenge::request::StatusRequest>,
) -> impl IntoResponse {
    match service::challenge::status(body).await {
        Ok(status) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(status),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
            })),
        ),
    }
}

pub async fn create(
    Json(body): Json<crate::model::challenge::request::CreateRequest>,
) -> impl IntoResponse {
    match service::challenge::create(body).await {
        Ok(challenge) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(challenge),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn update(
    Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<crate::model::challenge::request::UpdateRequest>,
) -> impl IntoResponse {
    body.id = Some(id);
    match service::challenge::update(body).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
            })),
        ),
        Err(_err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
            })),
        ),
    }
}

pub async fn delete(Path(id): Path<i64>) -> impl IntoResponse {
    match service::challenge::delete(id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16()
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
            })),
        ),
    }
}

pub async fn find_attachment(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(buffer.into())
                .unwrap();
        }
        None => return (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn find_attachment_metadata(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": {
                        "filename": filename,
                        "size": size,
                    },
                })),
            )
        }
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                        "code": StatusCode::NOT_FOUND.as_u16(),
                })),
            )
        }
    }
}

pub async fn save_attachment(Path(id): Path<i64>, mut multipart: Multipart) -> impl IntoResponse {
    let path = format!("challenges/{}/attachment", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "code": StatusCode::BAD_REQUEST.as_u16(),
                            "msg": "size_too_large",
                        })),
                    );
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    match crate::media::save(path, filename, data).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            );
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn delete_attachment(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("challenges/{}/attachment", id);

    match crate::media::delete(path).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "code": StatusCode::NOT_FOUND.as_u16(),
                })),
            )
        }
    }
}

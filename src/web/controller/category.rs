use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::model::category::request::{CreateRequest, FindRequest, UpdateRequest};
use crate::web::service;
use crate::util::validate;

/// **Find** can find categories.
///
/// ## Arguments
/// - `params`: category's information.
///
/// ## Returns
/// - `200`: find successfully.
/// - `400`: find failed.
pub async fn find(Query(params): Query<FindRequest>) -> impl IntoResponse {
    match service::category::find(params).await {
        Ok((categories, total)) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(categories),
                "total": total,
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

/// **Create** can create a new category.
///
/// ## Arguments
/// - `body`: category's information(validated).
///
/// ## Returns
/// - `200`: create successfully.
/// - `400`: create failed.
pub async fn create(validate::Json(body): validate::Json<CreateRequest>) -> impl IntoResponse {
    match service::category::create(body).await {
        Ok(category) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(category),
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

/// **Update** can update category's information.
///
/// ## Arguments
/// - `id`: category's id.
/// - `body`: category's information(validated).
///
/// ## Returns
/// - `200`: update successfully.
/// - `400`: update failed.
pub async fn update(Path(id): Path<i64>, validate::Json(mut body): validate::Json<UpdateRequest>) -> impl IntoResponse {
    body.id = Some(id);
    match service::category::update(body).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16()
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

/// **Delete** can be used to delete category.
///
/// ## Arguments
/// - `id`: category's id.
///
/// ## Returns
/// - `200`: delete successfully.
/// - `400`: delete failed.
pub async fn delete(Path(id): Path<i64>) -> impl IntoResponse {
    match service::category::delete(id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16()
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

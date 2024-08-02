use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use crate::web::service::submission as submission_service;
use crate::web::traits::Ext;

/// **Find** can be used to find submissions.
///
/// ## Arguments
/// - `params`: A `FindRequest` struct containing the parameters for the find operation.
/// - `ext`: An `Ext` struct containing the current operator.
///
/// ## Returns
/// - `200`: find successfully.
/// - `403`: operator does not have permission to find submissions.
/// - `400`: find failed.
pub async fn find(
    Extension(ext): Extension<Ext>,
    Query(params): Query<crate::model::submission::request::FindRequest>,
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

    match submission_service::find(params).await {
        Ok((submissions, total)) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(submissions),
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

pub async fn create(
    Extension(ext): Extension<Ext>,
    Json(mut body): Json<crate::model::submission::request::CreateRequest>,
) -> impl IntoResponse {
    let operator = ext.operator.unwrap();
    body.user_id = Some(operator.id);
    match submission_service::create(body).await {
        Ok(submission) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(submission),
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

// pub async fn update(
//     Path(id): Path<i64>,
//     validate::Json(mut body): validate::Json<UpdateRequest>,
// ) -> impl IntoResponse {
//     body.id = Some(id);
//     match submission_service::update(body).await {
//         Ok(()) => (
//             StatusCode::OK,
//             Json(json!({
//                 "code": StatusCode::OK.as_u16(),
//             })),
//         ),
//         Err(e) => (
//             StatusCode::BAD_REQUEST,
//             Json(json!({
//                 "code": StatusCode::BAD_REQUEST.as_u16(),
//             })),
//         ),
//     }
// }

pub async fn delete(Path(id): Path<i64>) -> impl IntoResponse {
    match submission_service::delete(id).await {
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

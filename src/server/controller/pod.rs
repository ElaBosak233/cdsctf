use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use crate::{server::service, traits::Ext};

pub async fn find(
    Query(params): Query<crate::model::pod::request::FindRequest>,
) -> impl IntoResponse {
    match service::pod::find(params).await {
        Ok((pods, total)) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(pods),
                "total": total,
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

pub async fn create(
    Extension(ext): Extension<Ext>,
    Json(mut body): Json<crate::model::pod::request::CreateRequest>,
) -> impl IntoResponse {
    let operator = ext.operator.clone().unwrap();
    body.user_id = Some(operator.id);

    match service::pod::create(body).await {
        Ok(pod) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(pod),
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

pub async fn update(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> impl IntoResponse {
    let operator = ext.operator.clone().unwrap();
    let (pods, total) = service::pod::find(crate::model::pod::request::FindRequest {
        id: Some(operator.id),
        ..Default::default()
    })
    .await
    .unwrap();

    let pod = pods
        .get(0)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "code": StatusCode::NOT_FOUND.as_u16(),
                })),
            )
        })
        .unwrap();

    if operator.group == "admin"
        || operator.id == pod.user_id
        || operator
            .teams
            .iter()
            .any(|team| Some(team.id) == pod.team_id)
    {
        match service::pod::update(id).await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                })),
            ),
        }
    } else {
        (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
            })),
        )
    }
}

pub async fn delete(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> impl IntoResponse {
    let operator = ext.operator.clone().unwrap();
    let (pods, total) = service::pod::find(crate::model::pod::request::FindRequest {
        id: Some(id),
        ..Default::default()
    })
    .await
    .unwrap();

    if total == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "code": StatusCode::NOT_FOUND.as_u16(),
            })),
        );
    }

    let pod = pods.get(0).unwrap();

    if operator.group == "admin"
        || operator.id == pod.user_id
        || operator
            .teams
            .iter()
            .any(|team| Some(team.id) == pod.team_id)
    {
        match service::pod::delete(id).await {
            Ok(_) => {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "code": StatusCode::OK.as_u16(),
                    })),
                )
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    })),
                )
            }
        }
    } else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
            })),
        );
    }
}

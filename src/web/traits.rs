use crate::model::user;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Clone, Debug)]
pub struct Ext {
    pub operator: Option<user::Model>,
}

#[derive(Debug, Error)]
pub enum WebError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("internal server error: {0}")]
    InternalServerError(String),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("too many requests: {0}")]
    TooManyRequests(String),
    #[error("database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("queue error: {0}")]
    QueueError(#[from] crate::queue::traits::QueueError),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            Self::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg.clone()),
            Self::DatabaseError(err) => match err {
                sea_orm::DbErr::RecordNotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            Self::QueueError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        return (
            status,
            Json(json!({
                "code": status.as_u16(),
                "msg": message,
            })),
        )
            .into_response();
    }
}

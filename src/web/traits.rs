use std::fmt::Debug;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Clone, Debug)]
pub struct Ext {
    pub operator: Option<crate::db::transfer::User>,
    pub client_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResult<T> {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    pub ts: i64,
}

impl<T> Default for WebResult<T> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: None,
            data: None,
            total: None,
            ts: 0,
        }
    }
}

impl<T: Serialize + Debug> IntoResponse for WebResult<T> {
    fn into_response(mut self) -> Response<Body> {
        self.ts = chrono::Utc::now().timestamp();
        (
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK),
            Json(self),
        )
            .into_response()
    }
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
    #[error("db error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("media error: {0}")]
    MediaError(#[from] crate::media::traits::MediaError),
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
            Self::MediaError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::QueueError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        WebResult::<()> {
            code: status.as_u16(),
            msg: Option::from(message),
            ..WebResult::default()
        }
        .into_response()
    }
}

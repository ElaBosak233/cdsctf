use std::fmt::Debug;

use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Clone, Debug, Default)]
pub struct Ext {
    pub operator: Option<cds_db::transfer::User>,
    pub client_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResponse<T> {
    #[serde(with = "http_serde::status_code")]
    pub code: StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<serde_json::Value>,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    pub ts: i64,
}

impl<T> Default for WebResponse<T> {
    fn default() -> Self {
        Self {
            code: StatusCode::OK,
            msg: None,
            data: None,
            total: None,
            ts: 0,
        }
    }
}

impl<T: Serialize + Debug> IntoResponse for WebResponse<T> {
    fn into_response(mut self) -> Response<Body> {
        self.ts = chrono::Utc::now().timestamp();
        (
            self.code,
            Json(self),
        )
            .into_response()
    }
}

#[derive(Debug, Error)]
pub enum WebError {
    #[error("not found: {0}")]
    NotFound(serde_json::Value),
    #[error("internal server error: {0}")]
    InternalServerError(serde_json::Value),
    #[error("bad request: {0}")]
    BadRequest(serde_json::Value),
    #[error("unauthorized: {0}")]
    Unauthorized(serde_json::Value),
    #[error("forbidden: {0}")]
    Forbidden(serde_json::Value),
    #[error("conflict: {0}")]
    Conflict(serde_json::Value),
    #[error("too many requests: {0}")]
    TooManyRequests(serde_json::Value),
    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(serde_json::Value),
    #[error("db error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("cache error: {0}")]
    CacheError(#[from] cds_cache::traits::CacheError),
    #[error("captcha error: {0}")]
    CaptchaError(#[from] cds_captcha::traits::CaptchaError),
    #[error("media error: {0}")]
    MediaError(#[from] cds_media::traits::MediaError),
    #[error("queue error: {0}")]
    QueueError(#[from] cds_queue::traits::QueueError),
    #[error("cluster error: {0}")]
    ClusterError(#[from] cds_cluster::traits::ClusterError),
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
            Self::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            Self::DatabaseError(err) => match err {
                sea_orm::DbErr::RecordNotFound(msg) => {
                    (StatusCode::NOT_FOUND, serde_json::json!(msg.clone()))
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    serde_json::json!(err.to_string()),
                ),
            },
            Self::CacheError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::CaptchaError(err) => match err {
                cds_captcha::traits::CaptchaError::Gone => {
                    (StatusCode::GONE, serde_json::json!(err.to_string()))
                }
                cds_captcha::traits::CaptchaError::MissingField(_) => {
                    (StatusCode::BAD_REQUEST, serde_json::json!(err.to_string()))
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    serde_json::json!(err.to_string()),
                ),
            },
            Self::MediaError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::QueueError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::ClusterError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::OtherError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
        };

        WebResponse::<()> {
            code: status,
            msg: Option::from(message),
            ..Default::default()
        }
        .into_response()
    }
}

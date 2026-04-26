//! Shared HTTP-layer types: application state [`AppState`], authenticated
//! caller [`AuthPrincipal`], common JSON envelopes, and [`WebError`] for
//! consistent API failures.

use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use cds_db::User;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, warn};

/// Process-wide state passed into Axum handlers (`State<Arc<AppState>>`).
#[derive(Clone)]
pub struct AppState {
    /// Merged TOML + environment configuration.
    pub env: cds_env::Env,
    /// NATS-backed pub/sub for live scoreboards and notifications.
    pub event: cds_event::EventManager,
    /// Primary PostgreSQL access.
    pub db: cds_db::DB,
    /// Redis JSON cache and session backing store.
    pub cache: cds_cache::Cache,
    /// Rune-based dynamic challenge checking.
    pub checker: cds_checker::Checker,
    /// Captcha provider facade.
    pub captcha: cds_captcha::Captcha,
    /// Kubernetes challenge runtime.
    pub cluster: cds_cluster::Cluster,
    /// S3-compatible object storage.
    pub media: cds_media::Media,
    /// SMTP outbound mail (also consumed from the `mailbox` queue).
    pub mailbox: cds_mailbox::Mailbox,
    /// NATS client for publishing background jobs.
    pub queue: cds_queue::Queue,
}

/// Authenticated (or anonymous) subject extracted by auth middleware before
/// handlers run.
#[derive(Clone, Debug, Default)]
pub struct AuthPrincipal {
    pub operator: Option<User>,
    pub client_ip: String,
}

/// JSON body for failed API responses. The HTTP status code is only on the
/// response line (not duplicated in this object).
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    /// Error detail; may be a string, object, or other JSON depending on the
    /// handler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<serde_json::Value>,
}

/// Empty JSON object (`{}`) for success responses that carry no fields.
#[derive(Debug, Default, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EmptyJson {}

/// Application error type converted into `(status, ErrorResponse)` for the
/// client.
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

    #[error("tower sessions error: {0}")]
    TowerSessionsError(#[from] tower_sessions::session::Error),
    #[error("http error: {0}")]
    HttpError(#[from] axum::http::Error),
    #[error("multipart error: {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),

    #[error("db error: {0}")]
    DbError(#[from] cds_db::traits::DbError),
    #[error("cache error: {0}")]
    CacheError(#[from] cds_cache::traits::CacheError),
    #[error("env error: {0}")]
    EnvError(#[from] cds_env::traits::EnvError),
    #[error("event error: {0}")]
    EventError(#[from] cds_event::traits::EventError),
    #[error("captcha error: {0}")]
    CaptchaError(#[from] cds_captcha::traits::CaptchaError),
    #[error("idp error: {0}")]
    IdpError(#[from] cds_idp::IdpError),
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
    /// Converts into response.
    fn into_response(self) -> Response<Body> {
        let error_kind = match &self {
            Self::NotFound(_) => "not_found",
            Self::InternalServerError(_) => "internal_server_error",
            Self::BadRequest(_) => "bad_request",
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
            Self::Conflict(_) => "conflict",
            Self::TooManyRequests(_) => "too_many_requests",
            Self::UnprocessableEntity(_) => "unprocessable_entity",
            Self::TowerSessionsError(_) => "tower_sessions",
            Self::HttpError(_) => "http",
            Self::MultipartError(_) => "multipart",
            Self::DbError(_) => "db",
            Self::CacheError(_) => "cache",
            Self::EnvError(_) => "env",
            Self::EventError(_) => "event",
            Self::CaptchaError(_) => "captcha",
            Self::IdpError(_) => "idp",
            Self::MediaError(_) => "media",
            Self::QueueError(_) => "queue",
            Self::ClusterError(_) => "cluster",
            Self::OtherError(_) => "other",
        };

        let (status, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            Self::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg.clone()),
            Self::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),

            Self::TowerSessionsError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::HttpError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::MultipartError(err) => {
                (StatusCode::BAD_REQUEST, serde_json::json!(err.to_string()))
            }

            Self::DbError(err) => match err {
                cds_db::traits::DbError::NotFound(msg) => {
                    (StatusCode::NOT_FOUND, serde_json::json!(msg.clone()))
                }
                _ => {
                    error!("{:?}", err);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        serde_json::json!(err.to_string()),
                    )
                }
            },
            Self::CacheError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::EnvError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!(err.to_string()),
            ),
            Self::EventError(err) => (
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
            Self::IdpError(err) => (StatusCode::BAD_REQUEST, serde_json::json!(err.to_string())),
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

        if status.is_server_error() {
            error!(
                error.kind = error_kind,
                status = status.as_u16(),
                error.message = %message,
                "request failed"
            );
        } else {
            warn!(
                error.kind = error_kind,
                status = status.as_u16(),
                error.message = %message,
                "request rejected"
            );
        }

        let body = ErrorResponse { msg: Some(message) };
        (status, Json(body)).into_response()
    }
}

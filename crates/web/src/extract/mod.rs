//! Thin wrappers around Axum extractors that map failures to
//! [`crate::traits::WebError`].
//!
//! Prefer these types in handlers for consistent JSON error bodies instead of
//! Axum’s default rejections.

use axum::{
    extract::{
        FromRequest, FromRequestParts, Request,
        path::ErrorKind,
        rejection::{ExtensionRejection, PathRejection, QueryRejection},
    },
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use garde::Validate;

use crate::traits::WebError;

/// Path parameters deserialized from the URL (400 on bad syntax, 500 on
/// framework gaps).
#[derive(Debug, Clone, Copy, Default)]
pub struct Path<T>(pub T);

impl<S, T> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = WebError;

    /// Builds `Self` from request parts.

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => match rejection {
                PathRejection::FailedToDeserializePathParams(inner) => {
                    let kind = inner.kind();
                    match &kind {
                        ErrorKind::UnsupportedType { .. } => {
                            Err(WebError::InternalServerError(json!("invalid_path")))
                        }
                        _ => Err(WebError::BadRequest(json!("invalid_path"))),
                    }
                }
                PathRejection::MissingPathParams(_) => {
                    Err(WebError::InternalServerError(json!("path_parameters_missing")))
                }
                _ => Err(WebError::InternalServerError(json!("path_extraction_failed"))),
            },
        }
    }
}

/// JSON body without extra validation (400 on deserialize failure).
#[derive(Debug, Clone, Copy, Default)]
pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = WebError;

    /// Builds `Self` from request.

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                tracing::debug!(error = %rejection, "invalid JSON request body");
                Err(WebError::BadRequest(json!("invalid_json")))
            }
        }
    }
}

/// JSON body plus `garde::Validate` (422 on validation errors).
#[derive(Debug, Clone, Copy, Default)]
pub struct VJson<T>(pub T);

impl<S, T> FromRequest<S> for VJson<T>
where
    T: Validate<Context = ()> + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = WebError;

    /// Builds `Self` from request.

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Json::<T>::from_request(req, state).await {
            Ok(value) => match value.0.validate() {
                Ok(_) => Ok(Self(value.0)),
                Err(validation_errors) => {
                    Err(WebError::UnprocessableEntity(json!({
                        "code": "validation_failed",
                        "errors": validation_errors
                    })))
                }
            },
            Err(rejection) => {
                tracing::debug!(error = %rejection, "invalid JSON request body");
                Err(WebError::BadRequest(json!("invalid_json")))
            }
        }
    }
}

/// Re-export pattern for Axum [`axum::extract::Extension`] with unified
/// [`WebError`] mapping.
#[derive(Debug, Clone, Copy, Default)]
pub struct Extension<T>(pub T);

impl<S, T> FromRequestParts<S> for Extension<T>
where
    T: Clone + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = WebError;

    /// Builds `Self` from request parts.

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Extension::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => match rejection {
                ExtensionRejection::MissingExtension(_) => {
                    Err(WebError::InternalServerError(json!("extension_missing")))
                }
                _ => Err(WebError::InternalServerError(json!("extension_extraction_failed"))),
            },
        }
    }
}

/// Query string deserialized into `T` (500 on malformed query for parity with
/// legacy behavior).
#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

impl<S, T> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = WebError;

    /// Builds `Self` from request parts.

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => match rejection {
                QueryRejection::FailedToDeserializeQueryString(_) => {
                    Err(WebError::BadRequest(json!("invalid_query")))
                }
                _ => Err(WebError::BadRequest(json!("query_extraction_failed"))),
            },
        }
    }
}

use async_trait::async_trait;
use axum::{
    extract::{path::ErrorKind, rejection::PathRejection, FromRequest, FromRequestParts},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::web::traits::WebError;

pub mod validate;

pub struct Path<T>(T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => match rejection {
                PathRejection::FailedToDeserializePathParams(inner) => {
                    let kind = inner.kind();
                    match &kind {
                        ErrorKind::UnsupportedType { .. } => {
                            Err(WebError::InternalServerError(kind.to_string()))
                        }
                        _ => Err(WebError::BadRequest(kind.to_string())),
                    }
                }
                PathRejection::MissingPathParams(error) => {
                    Err(WebError::InternalServerError(error.to_string()))
                }
                _ => Err(WebError::InternalServerError(rejection.to_string())),
            },
        }
    }
}

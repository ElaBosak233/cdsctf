//! Shared traits and error types for the `captcha` crate.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Default, Debug, Clone, utoipa::ToSchema)]
pub struct CaptchaChallenge {
    pub id: String,
    pub challenge: String,
    pub criteria: Option<String>,
}

impl CaptchaChallenge {
    /// Strips secrets so configuration can be returned to clients.
    pub fn desensitize(self) -> Self {
        Self {
            criteria: None,
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, utoipa::ToSchema)]
pub struct Answer {
    pub id: Option<String>,
    pub content: String,
    pub client_ip: Option<String>,
}

#[derive(Debug, Error)]
pub enum CaptchaError {
    #[error("gone")]
    Gone,
    #[error("missing field: {0}")]
    MissingField(String),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("biosvg error")]
    BiosvgError,
    #[error("cache error: {0}")]
    CacheError(#[from] cds_cache::traits::CacheError),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

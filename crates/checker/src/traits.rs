//! Shared traits and error types for the `checker` crate.

pub use cds_engine::traits::EngineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckerError {
    #[error("engine error: {0}")]
    EngineError(#[from] EngineError),
    #[error("missing script: {0}")]
    MissingScript(String),
    #[error("script error: {0}")]
    ScriptError(String),
    #[error("String UTF-8 decode error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    MediaError(#[from] cds_media::traits::MediaError),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

impl CheckerError {
    /// Admission failed before executing any script code; safe to defer.
    pub fn is_busy(&self) -> bool {
        matches!(self, Self::EngineError(EngineError::Busy))
    }
}

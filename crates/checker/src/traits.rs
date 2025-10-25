use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckerError {
    #[error("engine error: {0}")]
    EngineError(#[from] cds_engine::traits::EngineError),
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

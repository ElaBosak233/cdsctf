use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("internal server error: {0}")]
    InternalServerError(String),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

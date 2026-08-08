//! Shared traits and error types for the `env` crate.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("env file not found")]
    NotFound,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("figment error: {0}")]
    FigmentError(#[source] Box<figment::Error>),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

impl From<figment::Error> for EnvError {
    fn from(error: figment::Error) -> Self {
        Self::FigmentError(Box::new(error))
    }
}

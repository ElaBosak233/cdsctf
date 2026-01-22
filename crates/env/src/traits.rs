use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("env file not found")]
    NotFound,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("figment error: {0}")]
    FigmentError(#[from] figment::Error),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

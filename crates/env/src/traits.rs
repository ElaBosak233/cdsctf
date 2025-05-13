use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("env file not found")]
    NotFound,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("toml de error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("toml ser error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

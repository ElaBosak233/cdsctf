use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file not found")]
    ConfigNotFound,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("toml de error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

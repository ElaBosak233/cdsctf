use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("lettre error: {0}")]
    LettreError(#[from] lettre::error::Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

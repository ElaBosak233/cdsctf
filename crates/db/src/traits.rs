use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("sea_orm error: {0}")]
    SeaORM(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

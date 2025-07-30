use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("sea_orm error: {0}")]
    SeaORM(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

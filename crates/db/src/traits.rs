use thiserror::Error;

#[derive(Debug, Error)]
pub enum DBError {
    #[error("sea_orm error: {0}")]
    SeaORMError(#[from] sea_orm::DbErr),
}

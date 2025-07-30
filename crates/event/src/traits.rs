use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("queue error: {0}")]
    QueueError(#[from] cds_queue::traits::QueueError),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

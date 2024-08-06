use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("failed to connect to nats server: {0}")]
    ConnectError(#[from] async_nats::ConnectError),
    #[error("failed to serialize message: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("failed to publish message: {0}")]
    PublishError(#[from] async_nats::jetstream::context::PublishError),
    #[error("failed to construct consumer: {0}")]
    ConsumerError(#[from] async_nats::jetstream::stream::ConsumerError),
    #[error("failed to create consumer stream: {0}")]
    CreateStreamError(#[from] async_nats::jetstream::context::CreateStreamError),
    #[error("failed to get stream: {0}")]
    StreamError(#[from] async_nats::jetstream::consumer::StreamError),
}

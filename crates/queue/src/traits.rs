use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("connect error: {0}")]
    ConnectError(#[from] async_nats::ConnectError),
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("publish error: {0}")]
    PublishError(#[from] async_nats::jetstream::context::PublishError),
    #[error("consumer error: {0}")]
    ConsumerError(#[from] async_nats::jetstream::stream::ConsumerError),
    #[error("create stream error: {0}")]
    CreateStreamError(#[from] async_nats::jetstream::context::CreateStreamError),
    #[error("stream error: {0}")]
    StreamError(#[from] async_nats::jetstream::consumer::StreamError),
}

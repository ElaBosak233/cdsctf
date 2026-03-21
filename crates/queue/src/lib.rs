//! NATS / JetStream integration: publish messages and subscribe as pull
//! consumers.
//!
//! Workers call [`Queue::subscribe`] to process subjects such as `calculator`,
//! `mailbox`, or `events`. Each subject maps to a JetStream stream (created if
//! absent) with a durable consumer name.

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

pub use async_nats;
use cds_env::Env;
use serde::Serialize;
use tracing::info;
use traits::QueueError;

/// Holds the core NATS client and a JetStream context for streaming APIs.
#[derive(Clone, Debug)]
pub struct Queue {
    client: async_nats::Client,
    jet_stream: async_nats::jetstream::Context,
}

/// Connects with credentials from [`cds_env::Env::queue`] and wraps JetStream.
pub async fn init(env: &Env) -> Result<Queue, QueueError> {
    let client = async_nats::ConnectOptions::new()
        .require_tls(env.queue.tls)
        .user_and_password(env.queue.username.clone(), env.queue.password.clone())
        .token(env.queue.token.clone())
        .connect(format!("{}:{}", env.queue.host, env.queue.port))
        .await?;

    let jet_stream = async_nats::jetstream::new(client.clone());

    info!("Message queue initialized successfully.");

    Ok(Queue { client, jet_stream })
}

impl Queue {
    /// Serializes `payload` as JSON and publishes it on the JetStream
    /// `subject`.
    pub async fn publish(&self, subject: &str, payload: impl Serialize) -> Result<(), QueueError> {
        self.jet_stream
            .publish(subject.to_owned(), serde_json::to_string(&payload)?.into())
            .await?;

        Ok(())
    }

    /// Creates (if needed) a stream named `subject`, ensures a durable
    /// consumer, and returns a pull stream.
    ///
    /// `durable_name` identifies the consumer for replay; defaults to
    /// `"worker"` when `None`.
    pub async fn subscribe(
        &self,
        subject: &str,
        durable_name: Option<&str>,
    ) -> Result<async_nats::jetstream::consumer::pull::Stream, QueueError> {
        let stream = self
            .jet_stream
            .get_or_create_stream(async_nats::jetstream::stream::Config {
                name: String::from(subject),
                max_messages: 10_000,
                ..Default::default()
            })
            .await?;

        // Consumer name doubles as the durable name so restarts resume unacknowledged
        // messages.
        let subscriber = stream
            .get_or_create_consumer(
                subject,
                async_nats::jetstream::consumer::pull::Config {
                    durable_name: Some(durable_name.unwrap_or("worker").to_owned()),
                    ..Default::default()
                },
            )
            .await?;

        let messages = subscriber
            .stream()
            .max_messages_per_batch(10)
            .messages()
            .await?;

        Ok(messages)
    }

    /// Stops accepting new operations and waits for in-flight work to finish
    /// (`drain`).
    pub async fn shutdown(&self) -> Result<(), QueueError> {
        info!("Shutting down message queue...");

        self.client.drain().await?;
        Ok(())
    }
}

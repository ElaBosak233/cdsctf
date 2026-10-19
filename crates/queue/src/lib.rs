//! NATS / JetStream integration: publish messages and subscribe as pull
//! consumers.
//!
//! Workers call [`Queue::subscribe`] to process subjects such as
//! `cds.game.recalc`, `cds.mail.send`, or `cds.event.broadcast`. Each subject
//! maps to a JetStream stream (created if absent) with a durable consumer name.

use std::time::Duration;

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
    /// Publishes JSON and waits for the broker acknowledgement. The initiator
    /// is explicit business metadata, never read back from tracing fields.
    pub async fn publish(
        &self,
        subject: &str,
        payload: impl Serialize,
        initiator_id: Option<i64>,
    ) -> Result<(), QueueError> {
        let message_id = nanoid::nanoid!();
        let mut headers = async_nats::HeaderMap::new();
        headers.insert("X-Cds-Message-Id", message_id.as_str());
        if let Some(id) = initiator_id {
            headers.insert("X-Cds-Initiator-Id", id.to_string().as_str());
        }
        self.jet_stream
            .publish_with_headers(
                subject.to_owned(),
                headers,
                serde_json::to_vec(&payload)?.into(),
            )
            .await?
            .await?;
        info!(subject, message_id, initiator_id, "queue message published");
        Ok(())
    }

    /// Each delivery has its own native span. Message ID joins producer and
    /// consumer logs even when the OpenTelemetry exporter is disabled. Old
    /// messages still have their stream sequence, without a fabricated actor.
    pub fn message_span(message: &async_nats::jetstream::Message) -> tracing::Span {
        let message_id = message
            .headers
            .as_ref()
            .and_then(|headers| headers.get("X-Cds-Message-Id"))
            .map(|value| value.as_str());
        tracing::info_span!(parent: None, "queue.process",
            subject = %message.subject,
            message_id,
            stream_sequence = message.info().ok().map(|info| info.stream_sequence),
            operator_kind = "system",
            initiator_id = Self::initiator_id(message),
            submission_id = tracing::field::Empty,
        )
    }

    pub fn initiator_id(message: &async_nats::jetstream::Message) -> Option<i64> {
        message
            .headers
            .as_ref()
            .and_then(|headers| headers.get("X-Cds-Initiator-Id"))
            .and_then(|value| value.as_str().parse().ok())
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
        self.subscribe_inner(subject, durable_name, None).await
    }

    /// Subscribes with an explicit unacknowledged-message redelivery deadline.
    /// Existing durable consumers are updated so the requested deadline takes
    /// effect after deployment.
    pub async fn subscribe_with_ack_wait(
        &self,
        subject: &str,
        durable_name: Option<&str>,
        ack_wait: Duration,
    ) -> Result<async_nats::jetstream::consumer::pull::Stream, QueueError> {
        self.subscribe_inner(subject, durable_name, Some(ack_wait))
            .await
    }

    async fn subscribe_inner(
        &self,
        subject: &str,
        durable_name: Option<&str>,
        ack_wait: Option<Duration>,
    ) -> Result<async_nats::jetstream::consumer::pull::Stream, QueueError> {
        let stream = self
            .jet_stream
            .get_or_create_stream(async_nats::jetstream::stream::Config {
                name: subject.replace(['.', '_'], "-"),
                subjects: vec![subject.to_owned()],
                max_messages: 10_000,
                ..Default::default()
            })
            .await?;

        let durable_name = durable_name.unwrap_or("worker");
        let config = pull_consumer_config(subject, durable_name, ack_wait);
        let subscriber = if ack_wait.is_some() {
            // create_consumer is create-or-update, which applies a changed
            // ack_wait to a durable consumer that already exists.
            stream.create_consumer(config).await?
        } else {
            stream.get_or_create_consumer(durable_name, config).await?
        };

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

fn pull_consumer_config(
    subject: &str,
    durable_name: &str,
    ack_wait: Option<Duration>,
) -> async_nats::jetstream::consumer::pull::Config {
    async_nats::jetstream::consumer::pull::Config {
        filter_subject: subject.to_owned(),
        durable_name: Some(durable_name.to_owned()),
        ack_wait: ack_wait.unwrap_or_default(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_consumer_with_explicit_ack_wait() {
        let config = pull_consumer_config(
            "cds.submission.check",
            "worker",
            Some(Duration::from_secs(16)),
        );

        assert_eq!(config.filter_subject, "cds.submission.check");
        assert_eq!(config.durable_name.as_deref(), Some("worker"));
        assert_eq!(config.ack_wait, Duration::from_secs(16));
    }
}

//! NATS & JetStream integration:
//! - JetStream durable pull consumers ([`Queue::publish`] /
//!   [`Queue::subscribe`]) for `cds.mail.send` and `cds.event.broadcast`.
//! - Zero-overhead NATS Core signals ([`Queue::notify`],
//!   [`Queue::queue_subscribe`], [`Queue::subscribe_core`]) for stateless
//!   worker wakeups and cross-pod L1 cache invalidation.

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
    /// Sends a lightweight, non-persistent NATS Core signal (at-most-once).
    ///
    /// Used for work-queue wakeups and cache invalidation broadcasts where
    /// PostgreSQL or Valkey remains the durable source of truth.
    pub async fn notify(&self, subject: &str, payload: impl AsRef<[u8]>) -> Result<(), QueueError> {
        self.client
            .publish(subject.to_owned(), payload.as_ref().to_vec().into())
            .await?;
        Ok(())
    }

    /// Subscribes to a NATS Core subject in a load-balanced queue group so each
    /// signal wakes at most one subscriber across the cluster.
    pub async fn queue_subscribe(
        &self,
        subject: &str,
        queue_group: &str,
    ) -> Result<async_nats::Subscriber, QueueError> {
        Ok(self
            .client
            .queue_subscribe(subject.to_owned(), queue_group.to_owned())
            .await?)
    }

    /// Subscribes to a NATS Core broadcast subject so every pod receives each
    /// notification (e.g. L1 cache invalidation).
    pub async fn subscribe_core(
        &self,
        subject: &str,
    ) -> Result<async_nats::Subscriber, QueueError> {
        Ok(self.client.subscribe(subject.to_owned()).await?)
    }

    /// Publishes JSON to JetStream and waits for the broker acknowledgement.
    /// The initiator is explicit business metadata, never read back from
    /// tracing fields.
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
    /// consumer logs even when the OpenTelemetry exporter is disabled.
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

    /// Creates (if needed) a JetStream stream named `subject`, ensures a
    /// durable consumer, and returns a pull stream.
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
                name: subject.replace(['.', '_'], "-"),
                subjects: vec![subject.to_owned()],
                max_messages: 10_000,
                ..Default::default()
            })
            .await?;

        let durable_name = durable_name.unwrap_or("worker");
        let config = async_nats::jetstream::consumer::pull::Config {
            filter_subject: subject.to_owned(),
            durable_name: Some(durable_name.to_owned()),
            ..Default::default()
        };
        let subscriber = stream.get_or_create_consumer(durable_name, config).await?;

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

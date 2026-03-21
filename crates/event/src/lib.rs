//! Simple event bus on top of NATS JetStream (`events` subject).
//!
//! [`EventManager::push`] serializes [`types::Event`] as JSON;
//! [`EventManager::subscribe`] yields a stream of decoded events for WebSocket
//! or SSE style fan-out.

use std::convert::Infallible;

use cds_queue::Queue;
use futures_util::{Stream, StreamExt as _};
use tracing::info;

use crate::{traits::EventError, types::Event};

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Defines the `types` submodule (see sibling `*.rs` files).
pub mod types;

/// Thin wrapper around [`Queue`] for publishing and subscribing to `events`.
#[derive(Debug, Clone)]
pub struct EventManager {
    queue: Queue,
}

/// Filters for consumers; `token` selects a durable consumer name when
/// provided.
#[derive(Debug, Default)]
pub struct SubscribeOptions {
    pub game_id: Option<i64>,
    pub token: Option<String>,
}

/// Clones the queue handle into an [`EventManager`].
pub fn init(queue: &Queue) -> Result<EventManager, EventError> {
    info!("Event Manager was initialized successfully.");

    Ok(EventManager {
        queue: queue.clone(),
    })
}

impl EventManager {
    /// Publishes a single [`Event`] JSON payload on the fixed `events` subject.
    pub async fn push(&self, event: Event) -> Result<(), EventError> {
        self.queue.publish("events", event).await?;

        Ok(())
    }

    /// Long-lived stream of [`Event`] values decoded from JetStream messages;
    /// acks after handling each item.
    ///
    /// `SubscribeOptions::token` becomes the durable consumer name so multiple
    /// logical channels can coexist.
    pub async fn subscribe(
        &self,
        SubscribeOptions { game_id: _, token }: SubscribeOptions,
    ) -> Result<impl Stream<Item = Result<Event, Infallible>> + Send + use<>, EventError> {
        let mut messages = self.queue.subscribe("events", token.as_deref()).await?;

        let stream = async_stream::stream! {
            while let Some(Ok(message)) = messages.next().await {
                let payload = String::from_utf8(message.payload.to_vec()).unwrap_or("".to_owned());

                if let Ok(event) = serde_json::from_str::<Event>(&payload) {
                    yield Ok(event)
                }

                // Ack even when JSON is malformed so a poison message cannot block the consumer forever.
                let _ = message.ack().await;
            }
        };

        Ok(stream)
    }
}

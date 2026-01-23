use std::convert::Infallible;

use cds_queue::Queue;
use futures_util::{Stream, StreamExt as _};

use crate::{traits::EventError, types::Event};

pub mod traits;
pub mod types;

#[derive(Debug, Clone)]
pub struct EventManager {
    queue: Queue,
}

#[derive(Debug, Default)]
pub struct SubscribeOptions {
    pub game_id: Option<i64>,
    pub token: Option<String>,
}

pub fn init(queue: &Queue) -> Result<EventManager, EventError> {
    Ok(EventManager {
        queue: queue.clone(),
    })
}

impl EventManager {
    pub async fn push(&self, event: Event) -> Result<(), EventError> {
        self.queue.publish("events", event).await?;

        Ok(())
    }

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

                let _ = message.ack().await;
            }
        };

        Ok(stream)
    }
}

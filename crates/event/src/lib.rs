use std::convert::Infallible;

use futures_util::{Stream, StreamExt as _};

use crate::{traits::EventError, types::Event};

pub mod traits;
pub mod types;

pub async fn push(event: Event) -> Result<(), EventError> {
    cds_queue::publish("events", event).await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct SubscribeOptions {
    pub game_id: Option<i64>,
    pub token: Option<String>,
}

pub async fn subscribe(
    SubscribeOptions { game_id, token }: SubscribeOptions,
) -> Result<impl Stream<Item = Result<Event, Infallible>>, EventError> {
    let mut messages = cds_queue::subscribe("events", token.as_deref()).await?;

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

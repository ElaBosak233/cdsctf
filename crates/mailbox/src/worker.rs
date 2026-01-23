use futures_util::StreamExt as _;
use lettre::{Address, message::Mailbox as LMailbox};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::Mailbox;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

async fn process_messages(m: Mailbox) -> Result<(), anyhow::Error> {
    let mut messages = m.queue.subscribe("mailbox", None).await?;
    while let Some(Ok(message)) = messages.next().await {
        if let Ok(payload) = serde_json::from_slice::<Payload>(&message.payload) {
            match m
                .send(
                    LMailbox::new(
                        Some(payload.name.clone()),
                        payload.email.parse::<Address>()?,
                    ),
                    &payload.subject,
                    &payload.body,
                )
                .await
            {
                Ok(_) => {
                    info!(
                        name = payload.name,
                        email = payload.email,
                        "An email has been sent",
                    );
                }
                Err(err) => {
                    error!("Email send failed: {}", err);
                }
            };
        }
        message.double_ack().await.ok();
    }

    Ok(())
}

pub(crate) async fn init(m: Mailbox) {
    tokio::spawn(async move {
        if let Err(err) = process_messages(m.clone()).await {
            error!("{:?}", err);
        }
    });
}

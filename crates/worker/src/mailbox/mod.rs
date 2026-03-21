//! Consumer for JetStream subject **`mailbox`** (deliver outbound mail via
//! [`cds_mailbox::Mailbox`]).

use cds_mailbox::{Mailbox, Payload};
use cds_queue::Queue;
use futures_util::StreamExt as _;
use tracing::{error, info};

/// JetStream subject for queued outbound email.
pub const SUBJECT: &str = "mailbox";

async fn run(queue: Queue, mailbox: Mailbox) -> Result<(), anyhow::Error> {
    let mut messages = queue.subscribe(SUBJECT, None).await?;
    while let Some(Ok(message)) = messages.next().await {
        if let Ok(payload) = serde_json::from_slice::<Payload>(&message.payload) {
            match mailbox.send_payload(&payload).await {
                Ok(()) => {
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

/// Subscribes to [`SUBJECT`] and sends mail in a background task.
pub async fn spawn(queue: &Queue, mailbox: &Mailbox) {
    let queue = queue.clone();
    let mailbox = mailbox.clone();
    tokio::spawn(async move {
        if let Err(err) = run(queue, mailbox).await {
            error!("{:?}", err);
        }
    });

    info!(subject = SUBJECT, "queue consumer spawned");
}

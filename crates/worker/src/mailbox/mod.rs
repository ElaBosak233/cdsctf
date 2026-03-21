//! JetStream consumer for subject **`mailbox`**: deserializes
//! [`cds_mailbox::Payload`] and delivers mail through [`cds_mailbox::Mailbox`]
//! (SMTP settings from the database).
//!
//! Invalid JSON is skipped (still acked) so a poison message cannot block the
//! consumer indefinitely.

use cds_mailbox::{Mailbox, Payload};
use cds_queue::Queue;
use futures_util::StreamExt as _;
use tracing::{error, info};

/// Stream / subject name for asynchronous outbound email.
pub const SUBJECT: &str = "mailbox";

/// Blocking pull loop until the subscription ends or the process shuts down.
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
        // `double_ack` matches JetStream semantics used elsewhere in workers
        // (redelivery safety).
        message.double_ack().await.ok();
    }

    Ok(())
}

/// Spawns [`run`] on the Tokio runtime.
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

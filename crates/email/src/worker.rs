use futures_util::StreamExt as _;
use lettre::{Address, message::Mailbox};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::util;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

async fn process_messages() -> Result<(), anyhow::Error> {
    let mut messages = cds_queue::subscribe("email", None).await?;
    while let Some(Ok(message)) = messages.next().await {
        if let Ok(payload) = serde_json::from_slice::<Payload>(&message.payload) {
            match util::send(
                Mailbox::new(
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

pub(crate) async fn init() {
    tokio::spawn(async move {
        if let Err(err) = process_messages().await {
            error!("{:?}", err);
        }
    });
}

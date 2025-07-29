use futures::StreamExt as _;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{traits::EmailError, util};

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
            let to = format!("{} <{}>", payload.name, payload.email);
            match util::send(&to, &payload.subject, &payload.body).await {
                Ok(_) => {
                    info!("Email sent to {}", to);
                }
                Err(err) => {
                    error!("Email send failed: {}", err);
                }
            };
        }
        message.ack().await.unwrap();
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

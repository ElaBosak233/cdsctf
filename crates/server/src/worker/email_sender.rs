use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

pub async fn init() {
    tokio::spawn(async move {
        let mut messages = cds_queue::subscribe("email").await.unwrap();
        while let Some(Ok(message)) = messages.next().await {
            if let Ok(payload) = serde_json::from_slice::<Payload>(&message.payload) {
                let to = format!("{} <{}>", payload.name, payload.email);
                match cds_email::send(&to, &payload.subject, &payload.body).await {
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
    });
}

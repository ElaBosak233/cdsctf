pub mod traits;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::Serialize;
use tracing::info;
use traits::QueueError;

static CLIENT: OnceCell<async_nats::Client> = OnceCell::new();

fn get_client() -> &'static async_nats::Client {
    CLIENT.get().unwrap()
}

fn get_jetstream() -> async_nats::jetstream::Context {
    let client = get_client().to_owned();
    async_nats::jetstream::new(client)
}

pub async fn publish(subject: &'static str, payload: impl Serialize) -> Result<(), QueueError> {
    let jetstream = get_jetstream();

    jetstream
        .publish(subject, serde_json::to_string(&payload).unwrap().into())
        .await?;

    Ok(())
}

pub async fn subscribe(
    subject: &str,
) -> Result<async_nats::jetstream::consumer::pull::Stream, QueueError> {
    let jetstream = get_jetstream();

    let stream = jetstream
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: String::from(subject),
            max_messages: 10_000,
            ..Default::default()
        })
        .await?;

    let subscriber = stream
        .get_or_create_consumer(
            subject,
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some(String::from(subject)),
                ..Default::default()
            },
        )
        .await?;

    let messages = subscriber
        .stream()
        .max_messages_per_batch(10)
        .messages()
        .await?;

    Ok(messages)
}

pub async fn init() -> Result<(), QueueError> {
    let client = async_nats::ConnectOptions::new()
        .require_tls(cds_env::get_constant().queue.tls)
        .user_and_password(
            cds_env::get_constant().queue.username.clone(),
            cds_env::get_constant().queue.password.clone(),
        )
        .token(cds_env::get_constant().queue.token.clone())
        .connect(format!(
            "{}:{}",
            cds_env::get_constant().queue.host,
            cds_env::get_constant().queue.port
        ))
        .await?;
    CLIENT
        .set(client)
        .map_err(|_| anyhow!("Failed to set client into OnceCell."))?;

    info!("Message queue initialized successfully.");

    Ok(())
}

pub async fn shutdown() -> Result<(), QueueError> {
    info!("Shutting down message queue...");

    get_client().drain().await?;
    Ok(())
}

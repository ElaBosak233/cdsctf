pub mod traits;

use once_cell::sync::OnceCell;
use serde::Serialize;
use tracing::info;
use traits::QueueError;

static CLIENT: OnceCell<async_nats::Client> = OnceCell::new();

pub fn get_client() -> async_nats::Client {
    return CLIENT.get().unwrap().clone();
}

pub fn get_jetstream() -> async_nats::jetstream::Context {
    let client = get_client();
    return async_nats::jetstream::new(client);
}

pub async fn publish(subject: &'static str, payload: impl Serialize) -> Result<(), QueueError> {
    let jetstream = get_jetstream();

    jetstream
        .publish(subject, serde_json::to_string(&payload).unwrap().into())
        .await?;

    return Ok(());
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
            &subject,
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

    return Ok(messages);
}

pub async fn init() {
    let client = async_nats::ConnectOptions::new()
        .require_tls(crate::config::get_config().queue.tls)
        .user_and_password(
            crate::config::get_config().queue.user.clone(),
            crate::config::get_config().queue.password.clone(),
        )
        .token(crate::config::get_config().queue.token.clone())
        .connect(format!(
            "{}:{}",
            crate::config::get_config().queue.host,
            crate::config::get_config().queue.port
        ))
        .await
        .unwrap();
    CLIENT.set(client).unwrap();

    info!("Message queue initialized successfully.");
}

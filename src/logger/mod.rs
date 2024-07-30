use tracing::{info, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn init() {
    let filter = EnvFilter::from_default_env()
        .add_directive(Level::TRACE.into())
        .add_directive(Level::DEBUG.into())
        .add_directive("docker_api=info".parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_filter(filter);

    let file_appender = tracing_appender::rolling::daily("logs", "cds");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(file_layer)
        .init();

    info!("Logger initialized successfully.");
}

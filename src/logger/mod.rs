use once_cell::sync::OnceCell;
use tracing::{info, Level};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

static FILE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();
static CONSOLE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

pub async fn init() {
    let filter = EnvFilter::from_default_env()
        .add_directive(Level::TRACE.into())
        .add_directive("docker_api=info".parse().unwrap());

    let file_appender = tracing_appender::rolling::daily("logs", "cds");
    let (non_blocking_file, file_guard) = non_blocking(file_appender);
    let (non_blocking_console, console_guard) = non_blocking(std::io::stdout());
    let file_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(non_blocking_file)
        .with_ansi(false)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .json();

    let console_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(non_blocking_console)
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    info!("Logger initialized successfully.");

    FILE_GUARD.set(file_guard).unwrap();
    CONSOLE_GUARD.set(console_guard).unwrap();
}

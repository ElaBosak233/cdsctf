use std::path::Path;

use once_cell::sync::OnceCell;
use tracing::{Level, info};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

static FILE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();
static CONSOLE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

pub async fn init() -> Result<(), anyhow::Error> {
    let filter = EnvFilter::new(&cds_config::get_config().logger.level);

    let file_appender = rolling::RollingFileAppender::builder()
        .rotation(rolling::Rotation::DAILY)
        .filename_prefix("cdsctf")
        .filename_suffix("log")
        .max_log_files(cds_config::get_config().logger.files_kept.clone())
        .build(Path::new(&cds_config::get_config().logger.path).canonicalize()?)?;
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

    Ok(())
}

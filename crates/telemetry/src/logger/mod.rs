use anyhow::anyhow;
use once_cell::sync::OnceCell;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, WithExportConfig};
use opentelemetry_sdk::logs::{Logger, LoggerProvider};

pub static PROVIDER: OnceCell<LoggerProvider> = OnceCell::new();

pub fn get_provider() -> Option<LoggerProvider> {
    PROVIDER.get().map(|p| p.to_owned())
}

pub fn get_tracing_layer() -> Option<OpenTelemetryTracingBridge<LoggerProvider, Logger>> {
    get_provider().map(|p| OpenTelemetryTracingBridge::new(&p))
}

pub fn init() -> Result<(), anyhow::Error> {
    let log_exporter = LogExporter::builder()
        .with_tonic()
        .with_export_config(crate::get_export_config())
        .build()
        .map_err(|_| anyhow!("Failed to initialize log."))?;

    let logger_provider = LoggerProvider::builder()
        .with_batch_exporter(log_exporter, opentelemetry_sdk::runtime::Tokio)
        .with_resource(crate::RESOURCE.clone())
        .build();

    PROVIDER.set(logger_provider).ok();

    Ok(())
}

pub async fn shutdown() -> Result<(), anyhow::Error> {

    let handle = tokio::task::spawn_blocking(move || {
        for r in get_provider().unwrap().force_flush() {
            if let Err(e) = r {
                println!("unable to fully flush logs: {e}");
            }
        }
    });

    handle.await?;

    let handle = tokio::task::spawn_blocking(move || {
        get_provider().unwrap().shutdown().unwrap();
    });

    handle.await?;

    Ok(())
}

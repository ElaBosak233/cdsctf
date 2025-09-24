use anyhow::anyhow;
use once_cell::sync::OnceCell;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, WithExportConfig};
use opentelemetry_sdk::logs::{SdkLogger, SdkLoggerProvider};

pub static PROVIDER: OnceCell<SdkLoggerProvider> = OnceCell::new();

pub fn get_provider() -> Option<SdkLoggerProvider> {
    PROVIDER.get().map(|p| p.to_owned())
}

pub fn get_tracing_layer() -> Option<OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger>> {
    get_provider().map(|p| OpenTelemetryTracingBridge::new(&p))
}

pub fn init() -> Result<(), anyhow::Error> {
    let log_exporter = LogExporter::builder()
        .with_tonic()
        .with_export_config(crate::get_export_config())
        .build()
        .map_err(|_| anyhow!("Failed to initialize log."))?;

    let logger_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(crate::RESOURCE.clone())
        .build();

    PROVIDER.set(logger_provider).ok();

    Ok(())
}

pub async fn shutdown() -> Result<(), anyhow::Error> {
    tokio::task::spawn_blocking(move || {
        if let Err(e) = get_provider().unwrap().force_flush() {
            println!("unable to fully flush logs: {:?}", e);
        }
    })
    .await?;

    tokio::task::spawn_blocking(move || {
        if let Err(e) = get_provider().unwrap().shutdown() {
            println!("unable to shutdown telemetry logger provider: {:?}", e);
        }
    })
    .await?;

    Ok(())
}

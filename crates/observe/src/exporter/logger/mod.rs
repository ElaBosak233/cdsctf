use once_cell::sync::OnceCell;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, WithExportConfig};
use opentelemetry_sdk::logs::{SdkLogger, SdkLoggerProvider};

use crate::traits::ObserveError;

pub static PROVIDER: OnceCell<SdkLoggerProvider> = OnceCell::new();

pub fn get_provider() -> Result<SdkLoggerProvider, ObserveError> {
    Ok(PROVIDER
        .get()
        .map(|p| p.to_owned())
        .ok_or_else(|| ObserveError::NoInstance)?)
}

pub fn get_tracing_layer()
-> Result<OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger>, ObserveError> {
    let provider = get_provider()?;
    let bridge = OpenTelemetryTracingBridge::new(&provider);

    Ok(bridge)
}

pub fn init() -> Result<(), ObserveError> {
    let log_exporter = LogExporter::builder()
        .with_tonic()
        .with_export_config(super::get_export_config())
        .build()?;

    let logger_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(super::get_resource())
        .build();

    PROVIDER.set(logger_provider).ok();

    Ok(())
}

pub async fn shutdown() -> Result<(), ObserveError> {
    {
        let provider = get_provider()?;
        tokio::task::spawn_blocking(move || {
            if let Err(e) = provider.force_flush() {
                println!("unable to fully flush logs: {:?}", e);
            }
        })
        .await?;
    }

    {
        let provider = get_provider()?;
        tokio::task::spawn_blocking(move || {
            if let Err(e) = provider.shutdown() {
                println!("unable to shutdown telemetry logger provider: {:?}", e);
            }
        })
        .await?;
    }

    Ok(())
}

use cds_env::Env;
use once_cell::sync::OnceCell;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{Compression, LogExporter, Protocol, WithExportConfig, WithHttpConfig};
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

pub fn init(env: &Env) -> Result<(), ObserveError> {
    let log_ep = env
        .observe
        .exporter
        .log_endpoint
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(|| {
            env.observe
                .exporter
                .endpoint
                .as_deref()
                .filter(|s| !s.is_empty())
                .map(|ep| format!("{ep}/v1/logs"))
        });

    let log_ep = match log_ep {
        Some(v) => v,
        None => return Ok(()),
    };

    let log_exporter = LogExporter::builder()
        .with_http()
        .with_endpoint(log_ep.as_str())
        .with_protocol(Protocol::HttpBinary)
        .with_compression(Compression::Gzip)
        .build()?;

    let logger_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(super::get_resource(env))
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

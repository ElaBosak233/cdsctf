pub mod logger;
pub mod meter;
pub mod tracer;

use std::time::Duration;

use once_cell::sync::Lazy;
pub use opentelemetry;
pub use opentelemetry_otlp;
use opentelemetry_otlp::{ExportConfig, Protocol};
pub use opentelemetry_sdk;
use opentelemetry_sdk::Resource;
use tracing::info;

pub(crate) static RESOURCE: Lazy<Resource> =
    Lazy::new(|| Resource::builder().with_service_name("cdsctf").build());

pub(crate) fn get_export_config() -> ExportConfig {
    ExportConfig {
        endpoint: Some(cds_env::get_config().telemetry.endpoint_url.to_string()),
        timeout: Some(Duration::from_secs(5)),
        protocol: match cds_env::get_config().telemetry.protocol {
            cds_env::telemetry::Protocol::Json => Protocol::HttpJson,
            cds_env::telemetry::Protocol::Binary => Protocol::HttpBinary,
            cds_env::telemetry::Protocol::Grpc | cds_env::telemetry::Protocol::Unknown => {
                Protocol::Grpc
            }
        },
    }
}

pub async fn init() -> Result<(), anyhow::Error> {
    if !cds_env::get_config().telemetry.is_enabled {
        return Ok(());
    }

    meter::init()?;
    logger::init()?;
    tracer::init()?;

    Ok(())
}

pub async fn shutdown() -> Result<(), anyhow::Error> {
    if !cds_env::get_config().telemetry.is_enabled {
        return Ok(());
    }

    info!("Shutting down telemetry...");

    tracer::shutdown().await?;
    logger::shutdown().await?;

    Ok(())
}

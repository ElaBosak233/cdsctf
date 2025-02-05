pub mod logger;
pub mod meter;
pub mod tracer;

use std::time::Duration;

use anyhow::Context;
use once_cell::sync::Lazy;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use opentelemetry_sdk::Resource;

pub(crate) static RESOURCE: Lazy<Resource> = Lazy::new(|| {
    let pairs = vec![KeyValue::new("service.name", "cdsctf")];

    Resource::new(pairs)
});

pub async fn init() -> Result<(), anyhow::Error> {
    if !cds_config::get_config().telemetry.is_enabled {
        return Ok(());
    }

    meter::init()?;
    logger::init()?;
    tracer::init()?;

    Ok(())
}

pub fn get_export_config() -> ExportConfig {
    ExportConfig {
        endpoint: Some(cds_config::get_config().telemetry.endpoint_url.to_string()),
        timeout: Duration::from_secs(5),
        protocol: match cds_config::get_config().telemetry.protocol {
            cds_config::telemetry::Protocol::Json => Protocol::HttpJson,
            cds_config::telemetry::Protocol::Binary => Protocol::HttpBinary,
            cds_config::telemetry::Protocol::Grpc | cds_config::telemetry::Protocol::Unknown => {
                Protocol::Grpc
            }
        },
    }
}

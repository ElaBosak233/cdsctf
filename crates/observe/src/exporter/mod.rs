use std::time::Duration;

use opentelemetry_otlp::{ExportConfig, Protocol};
use opentelemetry_sdk::Resource;

use crate::traits::ObserveError;

pub mod logger;
pub mod meter;
pub mod tracer;

pub(crate) fn get_export_config() -> ExportConfig {
    ExportConfig {
        endpoint: Some(
            cds_env::get_config()
                .observe
                .exporter
                .endpoint_url
                .to_string(),
        ),
        timeout: Some(Duration::from_secs(5)),
        protocol: match cds_env::get_config().observe.exporter.protocol {
            cds_env::observe::exporter::Protocol::Json => Protocol::HttpJson,
            cds_env::observe::exporter::Protocol::Binary => Protocol::HttpBinary,
            cds_env::observe::exporter::Protocol::Grpc
            | cds_env::observe::exporter::Protocol::Unknown => Protocol::Grpc,
        },
    }
}

pub(crate) fn get_resource() -> Resource {
    Resource::builder()
        .with_service_name(cds_env::get_config().observe.service_name.clone())
        .build()
}

pub fn init() -> Result<(), ObserveError> {
    if !cds_env::get_config().observe.exporter.is_enabled {
        return Ok(());
    }

    logger::init()?;
    meter::init()?;
    tracer::init()?;

    Ok(())
}

pub async fn shutdown() -> Result<(), ObserveError> {
    if !cds_env::get_config().observe.exporter.is_enabled {
        return Ok(());
    }

    logger::shutdown().await?;
    tracer::shutdown().await?;

    Ok(())
}

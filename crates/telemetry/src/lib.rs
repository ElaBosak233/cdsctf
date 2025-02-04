pub mod metric;

use std::time::Duration;

use anyhow::{Context, anyhow};
use opentelemetry::global;
use opentelemetry_otlp::{ExportConfig, MetricExporter, Protocol, WithExportConfig};
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider, Temporality};

pub async fn init() -> Result<(), anyhow::Error> {
    if !cds_config::get_config().telemetry.is_enabled {
        return Ok(());
    }

    let export_config = ExportConfig {
        endpoint: Some(cds_config::get_config().telemetry.endpoint_url.to_string()),
        timeout: Duration::from_secs(5),
        protocol: match cds_config::get_config().telemetry.protocol {
            cds_config::telemetry::Protocol::Json => Protocol::HttpJson,
            cds_config::telemetry::Protocol::Binary => Protocol::HttpBinary,
            cds_config::telemetry::Protocol::Grpc | cds_config::telemetry::Protocol::Unknown => {
                Protocol::Grpc
            }
        },
    };

    let metric_exporter = MetricExporter::builder()
        .with_temporality(Temporality::Cumulative)
        .with_tonic()
        .with_export_config(export_config)
        .build()
        .map_err(|_| anyhow!("Failed to initialize metrics"))?;

    let meter_provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(metric_exporter, opentelemetry_sdk::runtime::Tokio)
                .with_interval(Duration::from_secs(3))
                .build(),
        )
        .with_resource(metric::METRICS_RESOURCE.clone())
        .build();

    global::set_meter_provider(meter_provider);

    Ok(())
}

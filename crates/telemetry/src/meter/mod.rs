mod system;
pub mod web;

use std::{borrow::Cow, time::Duration};

use anyhow::anyhow;
use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{InstrumentationScope, global, metrics::Meter};
use opentelemetry_otlp::{MetricExporter, WithExportConfig};
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider, Temporality};

pub static PROVIDER: OnceCell<SdkMeterProvider> = OnceCell::new();

pub static METER: Lazy<Meter> = Lazy::new(|| {
    let scope = InstrumentationScope::builder("cdsctf")
        .with_version(Cow::Borrowed(env!("CARGO_PKG_VERSION")))
        .build();

    global::meter_with_scope(scope)
});

pub fn init() -> Result<(), anyhow::Error> {
    let metric_exporter = MetricExporter::builder()
        .with_temporality(Temporality::Cumulative)
        .with_tonic()
        .with_export_config(crate::get_export_config())
        .build()
        .map_err(|_| anyhow!("Failed to initialize metrics."))?;

    let meter_provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(metric_exporter)
                .with_interval(Duration::from_secs(3))
                .build(),
        )
        .with_resource(crate::RESOURCE.clone())
        .build();

    PROVIDER.set(meter_provider).ok();
    global::set_meter_provider(PROVIDER.get().unwrap().to_owned());

    system::init_cpu_usage_observable_gauge();
    system::init_ram_usage_observable_gauge();

    web::init_active_requests();
    web::init_request_bytes();
    web::init_response_bytes();

    Ok(())
}

mod system;
pub mod web;

use std::{borrow::Cow, time::Duration};

use cds_env::Env;
use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{InstrumentationScope, global, metrics::Meter};
use opentelemetry_otlp::{Compression, MetricExporter, Protocol, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider, Temporality};

use crate::traits::ObserveError;

pub static PROVIDER: OnceCell<SdkMeterProvider> = OnceCell::new();

fn get_provider() -> SdkMeterProvider {
    PROVIDER
        .get()
        .expect("No meter provider instance, forget to init?")
        .to_owned()
}

pub static METER: Lazy<Meter> = Lazy::new(|| {
    let scope = InstrumentationScope::builder("cdsctf")
        .with_version(Cow::Borrowed(env!("CARGO_PKG_VERSION")))
        .build();

    global::meter_with_scope(scope)
});

pub fn init(env: &Env) -> Result<(), ObserveError> {
    let metric_ep = env
        .observe
        .exporter
        .metric_endpoint
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(|| {
            env.observe
                .exporter
                .endpoint
                .as_deref()
                .filter(|s| !s.is_empty())
                .map(|ep| format!("{ep}/v1/metrics"))
        });

    let metric_ep = match metric_ep {
        Some(v) => v,
        None => return Ok(()),
    };

    let metric_exporter = MetricExporter::builder()
        .with_temporality(Temporality::Cumulative)
        .with_http()
        .with_endpoint(metric_ep.as_str())
        .with_protocol(Protocol::HttpBinary)
        .with_compression(Compression::Gzip)
        .build()?;

    let meter_provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(metric_exporter)
                .with_interval(Duration::from_secs(3))
                .build(),
        )
        .with_resource(super::get_resource(env))
        .build();

    PROVIDER.set(meter_provider).ok();
    global::set_meter_provider(get_provider());

    system::init_cpu_usage_observable_gauge();
    system::init_ram_usage_observable_gauge();

    Ok(())
}

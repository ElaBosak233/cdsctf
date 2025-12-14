mod system;
pub mod web;

use std::{borrow::Cow, time::Duration};

use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{InstrumentationScope, global, metrics::Meter};
use opentelemetry_otlp::{MetricExporter, WithExportConfig};
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

pub fn init() -> Result<(), ObserveError> {
    let metric_exporter = MetricExporter::builder()
        .with_temporality(Temporality::Cumulative)
        .with_tonic()
        .with_export_config(super::get_export_config())
        .build()?;

    let meter_provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(metric_exporter)
                .with_interval(Duration::from_secs(3))
                .build(),
        )
        .with_resource(super::get_resource())
        .build();

    PROVIDER.set(meter_provider).ok();
    global::set_meter_provider(get_provider());

    system::init_cpu_usage_observable_gauge();
    system::init_ram_usage_observable_gauge();

    Ok(())
}

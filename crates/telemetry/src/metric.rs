use std::borrow::Cow;

use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{
    InstrumentationScope, KeyValue, global,
    metrics::{Meter, ObservableGauge},
};
use opentelemetry_sdk::{Resource, metrics::SdkMeterProvider};
use sysinfo::System;

pub static METRICS_RESOURCE: Lazy<Resource> = Lazy::new(|| {
    let pairs = vec![KeyValue::new("service.name", "cdsctf")];

    Resource::new(pairs)
});

pub static METER: Lazy<Meter> = Lazy::new(|| {
    let scope = InstrumentationScope::builder("cdsctf")
        .with_version(Cow::Borrowed(env!("CARGO_PKG_VERSION")))
        .build();

    global::meter_with_scope(scope)
});

pub static CPU_USAGE_OBSERVABLE_GAUGE: Lazy<ObservableGauge<f64>> = Lazy::new(|| {
    METER
        .f64_observable_gauge("cpu_usage")
        .with_description("CPU usage")
        .with_callback(|observer| {
            let mut system = System::new();
            let pid = sysinfo::get_current_pid().expect("Failed to get current process's PID");
            let measurement = if let Some(process) = system.process(pid) {
                process.cpu_usage() as f64
            } else {
                0.0
            };
            observer.observe(measurement, &[])
        })
        .build()
});

pub static RAM_USAGE_OBSERVABLE_GAUGE: Lazy<ObservableGauge<u64>> = Lazy::new(|| {
    METER
        .u64_observable_gauge("ram_usage")
        .with_description("RAM usage")
        .with_callback(|observer| {
            let mut system = System::new();
            let pid = sysinfo::get_current_pid().expect("Failed to get current process's PID");
            let measurement = if let Some(process) = system.process(pid) {
                process.memory()
            } else {
                0
            };
            observer.observe(measurement, &[])
        })
        .build()
});

use once_cell::sync::Lazy;
use prometheus::{Gauge, IntCounterVec, IntGauge, Opts, Registry};

pub static METRICS_REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

pub static HTTP_REQUEST_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    let opts = Opts::new(
        "http_requests_total",
        "Total number of HTTP requests received",
    )
    .namespace(cds_env::get_env().metric.namespace.clone());
    let counter = IntCounterVec::new(opts, &["method", "path"]).expect("metric can be created");
    METRICS_REGISTRY
        .register(Box::new(counter.clone()))
        .unwrap();
    counter
});

pub static MEMORY_USAGE: Lazy<IntGauge> = Lazy::new(|| {
    let opts = Opts::new("memory_usage_bytes", "Memory usage in bytes")
        .namespace(cds_env::get_env().metric.namespace.clone());
    let gauge = IntGauge::with_opts(opts).expect("metric can be created");
    METRICS_REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

pub static MEMORY_USAGE_RATIO: Lazy<Gauge> = Lazy::new(|| {
    let opts = Opts::new("memory_usage_ratio", "Memory usage ratio")
        .namespace(cds_env::get_env().metric.namespace.clone());
    let gauge = Gauge::with_opts(opts).expect("metric can be created");
    METRICS_REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

pub static CPU_USAGE: Lazy<IntGauge> = Lazy::new(|| {
    let opts = Opts::new("cpu_usage_percent", "CPU usage percentage")
        .namespace(cds_env::get_env().metric.namespace.clone());
    let gauge = IntGauge::with_opts(opts).expect("metric can be created");
    METRICS_REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

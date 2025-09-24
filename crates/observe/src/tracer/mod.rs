use std::borrow::Cow;

use anyhow::anyhow;
use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{
    InstrumentationScope, global, global::BoxedTracer,
    trace::TracerProvider as TracerProviderTraits,
};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::trace::{SdkTracerProvider, Tracer};

static PROVIDER: OnceCell<SdkTracerProvider> = OnceCell::new();

pub fn get_provider() -> Option<SdkTracerProvider> {
    PROVIDER.get().map(|p| p.to_owned())
}

pub fn get_tracer() -> Tracer {
    get_provider().unwrap().tracer("cdsctf")
}

pub static TRACER: Lazy<BoxedTracer> = Lazy::new(|| {
    let scope = InstrumentationScope::builder("cdsctf")
        .with_version(Cow::Borrowed(env!("CARGO_PKG_VERSION")))
        .build();

    global::tracer_with_scope(scope)
});

pub fn init() -> Result<(), anyhow::Error> {
    let span_exporter = SpanExporter::builder()
        .with_tonic()
        .with_export_config(crate::get_export_config())
        .build()
        .map_err(|_| anyhow!("Failed to initialize span."))?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(crate::RESOURCE.clone())
        .build();

    PROVIDER.set(tracer_provider).ok();
    global::set_tracer_provider(get_provider().unwrap());

    Ok(())
}

pub async fn shutdown() -> Result<(), anyhow::Error> {
    tokio::task::spawn_blocking(move || {
        if let Err(e) = get_provider().unwrap().force_flush() {
            println!("unable to fully flush traces: {:?}", e);
        }
    })
    .await?;

    tokio::task::spawn_blocking(move || {
        if let Err(e) = get_provider().unwrap().shutdown() {
            println!("unable to shutdown telemetry tracer provider: {:?}", e);
        }
    })
    .await?;

    Ok(())
}

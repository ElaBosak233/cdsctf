use std::borrow::Cow;

use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{
    InstrumentationScope, global, global::BoxedTracer,
    trace::TracerProvider as TracerProviderTraits,
};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::trace::{SdkTracerProvider, Tracer};

use crate::traits::ObserveError;

static PROVIDER: OnceCell<SdkTracerProvider> = OnceCell::new();

pub fn get_provider() -> Result<SdkTracerProvider, ObserveError> {
    PROVIDER
        .get()
        .map(|p| p.to_owned())
        .ok_or_else(|| ObserveError::NoInstance)
}

pub fn get_tracer() -> Result<Tracer, ObserveError> {
    Ok(get_provider()?.tracer("cdsctf"))
}

pub static TRACER: Lazy<BoxedTracer> = Lazy::new(|| {
    let scope = InstrumentationScope::builder("cdsctf")
        .with_version(Cow::Borrowed(env!("CARGO_PKG_VERSION")))
        .build();

    global::tracer_with_scope(scope)
});

pub fn init() -> Result<(), ObserveError> {
    let span_exporter = SpanExporter::builder()
        .with_tonic()
        .with_export_config(crate::get_export_config())
        .build()?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(crate::RESOURCE.clone())
        .build();

    PROVIDER.set(tracer_provider).ok();
    global::set_tracer_provider(get_provider()?);

    Ok(())
}

pub async fn shutdown() -> Result<(), ObserveError> {
    {
        let provider = get_provider()?;
        tokio::task::spawn_blocking(move || {
            if let Err(e) = provider.force_flush() {
                println!("unable to fully flush traces: {:?}", e);
            }
        })
        .await?;
    }

    {
        let provider = get_provider()?;
        tokio::task::spawn_blocking(move || {
            if let Err(e) = provider.shutdown() {
                println!("unable to shutdown telemetry tracer provider: {:?}", e);
            }
        })
        .await?;
    }

    Ok(())
}

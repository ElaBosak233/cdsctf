use std::borrow::Cow;

use cds_env::Env;
use once_cell::sync::{Lazy, OnceCell};
use opentelemetry::{
    InstrumentationScope, global, global::BoxedTracer,
    trace::TracerProvider as TracerProviderTraits,
};
use opentelemetry_otlp::{Compression, Protocol, SpanExporter, WithExportConfig, WithHttpConfig};
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

pub fn init(env: &Env) -> Result<(), ObserveError> {
    let trace_ep = env
        .observe
        .exporter
        .trace_endpoint
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(|| {
            env.observe
                .exporter
                .endpoint
                .as_deref()
                .filter(|s| !s.is_empty())
                .map(|ep| format!("{ep}/v1/traces"))
        });

    let trace_ep = match trace_ep {
        Some(v) => v,
        None => return Ok(()),
    };

    let span_exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(trace_ep)
        .with_protocol(Protocol::HttpBinary)
        .with_compression(Compression::Gzip)
        .build()?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(super::get_resource(env))
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

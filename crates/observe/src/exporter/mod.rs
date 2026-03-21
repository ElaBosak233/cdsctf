//! Observability — `mod` (metrics, tracing, or logging glue).

use cds_env::Env;
use opentelemetry_sdk::Resource;

use crate::traits::ObserveError;

/// Defines the `logger` submodule (see sibling `*.rs` files).
pub mod logger;

/// Defines the `meter` submodule (see sibling `*.rs` files).
pub mod meter;

/// Defines the `tracer` submodule (see sibling `*.rs` files).
pub mod tracer;

/// Returns resource.

pub(crate) fn get_resource(env: &Env) -> Resource {
    Resource::builder()
        .with_service_name(env.observe.service_name.clone())
        .build()
}

/// Initializes this subsystem or resource.

pub fn init(env: &Env) -> Result<(), ObserveError> {
    if !env.observe.exporter.enabled {
        return Ok(());
    }

    logger::init(env)?;
    meter::init(env)?;
    tracer::init(env)?;

    Ok(())
}

/// Flushes exporters and tears down observability integrations.
pub async fn shutdown(env: &Env) -> Result<(), ObserveError> {
    if !env.observe.exporter.enabled {
        return Ok(());
    }

    logger::shutdown().await?;
    tracer::shutdown().await?;

    Ok(())
}

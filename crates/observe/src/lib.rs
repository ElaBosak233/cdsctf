//! Observability wiring: OpenTelemetry exporters and tracing subscriber setup.
//!
//! [`init`] runs early in process startup; [`shutdown`] flushes telemetry
//! during graceful exit.

/// Defines the `exporter` submodule (see sibling `*.rs` files).
pub mod exporter;

/// Defines the `logger` submodule (see sibling `*.rs` files).
pub mod logger;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

use cds_env::Env;
use tracing::info;

use crate::traits::ObserveError;

/// Installs metrics/tracing exporters according to `env.observe`.
pub async fn init(env: &Env) -> Result<(), ObserveError> {
    exporter::init(env)?;
    logger::init(env).await?;

    Ok(())
}

/// Best-effort shutdown hook to flush pending OTLP batches, etc.
pub async fn shutdown(env: &Env) -> Result<(), ObserveError> {
    info!("Shutting down observability...");
    exporter::shutdown(env).await?;

    Ok(())
}

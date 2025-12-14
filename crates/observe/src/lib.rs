pub mod exporter;
pub mod logger;
pub mod traits;

use tracing::info;

use crate::traits::ObserveError;

pub async fn init() -> Result<(), ObserveError> {
    exporter::init()?;
    logger::init().await?;

    Ok(())
}

pub async fn shutdown() -> Result<(), ObserveError> {
    info!("Shutting down observability...");
    exporter::shutdown().await?;

    Ok(())
}

pub mod exporter;
pub mod logger;
pub mod traits;

use cds_env::Env;
use tracing::info;

use crate::traits::ObserveError;

pub async fn init(env: &Env) -> Result<(), ObserveError> {
    exporter::init(env)?;
    logger::init(env).await?;

    Ok(())
}

pub async fn shutdown(env: &Env) -> Result<(), ObserveError> {
    info!("Shutting down observability...");
    exporter::shutdown(env).await?;

    Ok(())
}

use opentelemetry_sdk::Resource;

use crate::traits::ObserveError;

pub mod logger;
pub mod meter;
pub mod tracer;

pub(crate) fn get_resource() -> Resource {
    Resource::builder()
        .with_service_name(cds_env::get_config().observe.service_name.clone())
        .build()
}

pub fn init() -> Result<(), ObserveError> {
    if !cds_env::get_config().observe.exporter.enabled {
        return Ok(());
    }

    logger::init()?;
    meter::init()?;
    tracer::init()?;

    Ok(())
}

pub async fn shutdown() -> Result<(), ObserveError> {
    if !cds_env::get_config().observe.exporter.enabled {
        return Ok(());
    }

    logger::shutdown().await?;
    tracer::shutdown().await?;

    Ok(())
}

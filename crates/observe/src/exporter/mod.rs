use cds_env::Env;
use opentelemetry_sdk::Resource;

use crate::traits::ObserveError;

pub mod logger;
pub mod meter;
pub mod tracer;

pub(crate) fn get_resource(env: &Env) -> Resource {
    Resource::builder()
        .with_service_name(env.observe.service_name.clone())
        .build()
}

pub fn init(env: &Env) -> Result<(), ObserveError> {
    if !env.observe.exporter.enabled {
        return Ok(());
    }

    logger::init(env)?;
    meter::init(env)?;
    tracer::init(env)?;

    Ok(())
}

pub async fn shutdown(env: &Env) -> Result<(), ObserveError> {
    if !env.observe.exporter.enabled {
        return Ok(());
    }

    logger::shutdown().await?;
    tracer::shutdown().await?;

    Ok(())
}

pub mod extract;
pub mod middleware;
pub mod model;
pub mod router;
pub mod traits;
pub mod util;

use std::{sync::Arc, time::Duration};

use axum::Router;
use once_cell::sync::OnceCell;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::{Any, CorsLayer};

use crate::middleware::{error::governor_error, network::GovernorKeyExtractor};

static APP: OnceCell<Router> = OnceCell::new();

pub async fn init() -> Result<(), anyhow::Error> {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(500)
            .burst_size(32)
            .key_extractor(GovernorKeyExtractor)
            .use_headers()
            .error_handler(governor_error)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            governor_limiter.retain_recent();
        }
    });

    let router = router::router().await.layer(cors).layer(GovernorLayer {
        config: governor_conf,
    });

    APP.set(router)
        .map_err(|_| anyhow::anyhow!("Failed to set router into OnceCell"))?;

    Ok(())
}

pub fn get_app() -> &'static Router {
    &APP.get().unwrap()
}

pub mod extract;
pub mod middleware;
pub mod model;
pub mod router;
pub mod traits;
pub mod util;
mod worker;

use axum::Router;
use once_cell::sync::OnceCell;
use tower_http::cors::{Any, CorsLayer};

static APP: OnceCell<Router> = OnceCell::new();

pub async fn init() -> Result<(), anyhow::Error> {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let router = router::router().await.layer(cors);

    APP.set(router)
        .map_err(|_| anyhow::anyhow!("Failed to set router into OnceCell"))?;

    worker::init().await?;

    Ok(())
}

pub fn get_app() -> &'static Router {
    APP.get().unwrap()
}

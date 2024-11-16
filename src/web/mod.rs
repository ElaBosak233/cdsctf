pub mod middleware;
pub mod model;
pub mod router;
pub mod traits;

use std::sync::OnceLock;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

static APP: OnceLock<Router> = OnceLock::new();

pub async fn init() {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let router = router::router().await.layer(cors);

    APP.set(router).unwrap();
}

pub fn get_app() -> Router {
    APP.get().unwrap().clone()
}

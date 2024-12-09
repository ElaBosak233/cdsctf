pub mod extract;
pub mod middleware;
pub mod model;
pub mod router;
pub mod traits;
pub mod util;

use axum::Router;
use once_cell::sync::OnceCell;
use tower_http::cors::{Any, CorsLayer};

static APP: OnceCell<Router> = OnceCell::new();

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

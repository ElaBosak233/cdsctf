pub mod controller;
pub mod middleware;
pub mod router;
pub mod service;
pub mod traits;

use std::sync::OnceLock;

use axum::{middleware::from_fn, Router};
use reqwest::Method;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

static APP: OnceLock<Router> = OnceLock::new();

pub fn init() {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    let app: Router = Router::new()
        .merge(
            Router::new()
                .nest("/api", router::router())
                .layer(TraceLayer::new_for_http()),
        )
        .layer(from_fn(middleware::frontend::serve))
        .layer(cors);

    APP.set(app).unwrap();
}

pub fn get_app() -> Router {
    return APP.get().unwrap().clone();
}

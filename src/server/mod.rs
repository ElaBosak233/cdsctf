pub mod controller;
pub mod middleware;
pub mod router;
pub mod service;

use axum::Router;
use reqwest::Method;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;

use crate::{config, container, database, logger, util};

pub async fn bootstrap() {
    logger::init();
    config::init().await;
    database::init().await;
    container::init().await;

    info!("{:?}", util::jwt::get_secret().await);

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
        .merge(Router::new().fallback_service(
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        ))
        .layer(cors);

    let addr = format!(
        "{}:{}",
        config::get_app_config().axum.host,
        config::get_app_config().axum.port
    );

    let listener = tokio::net::TcpListener::bind(&addr).await;
    info!(
        "Cloudsdale service has been started at {}. Enjoy your hacking challenges!",
        &addr
    );
    axum::serve(listener.unwrap(), app).await.unwrap();
}

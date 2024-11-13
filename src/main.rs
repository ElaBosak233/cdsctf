mod assets;
mod cache;
mod captcha;
mod cluster;
mod config;
mod database;
mod email;
mod logger;
mod media;
mod model;
mod queue;
mod util;
mod web;
mod metric;

use std::net::SocketAddr;

use tracing::info;

#[tokio::main]
async fn main() {
    let banner = assets::get("banner.txt").unwrap();
    println!(
        "{}",
        std::str::from_utf8(&banner)
            .unwrap()
            .replace("{{version}}", env!("CARGO_PKG_VERSION"))
            .replace("{{git_commit}}", env!("GIT_COMMIT"))
            .replace("{{build_at}}", env!("BUILD_AT"))
    );

    bootstrap().await;
}

async fn bootstrap() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("");

    logger::init().await;
    config::init().await;
    database::init().await;
    queue::init().await;
    cache::init().await;
    cluster::init().await;
    web::init().await;

    let addr = format!(
        "{}:{}",
        config::get_config().axum.host,
        config::get_config().axum.port
    );
    let listener = tokio::net::TcpListener::bind(&addr).await;

    info!(
        "CdsCTF service has been started at {}. Enjoy your hacking challenges!",
        &addr
    );

    axum::serve(
        listener.unwrap(),
        web::get_app().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

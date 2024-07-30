mod assets;
mod captcha;
mod config;
mod container;
mod database;
mod email;
mod logger;
mod media;
mod model;
mod proxy;
mod traits;
mod util;
mod web;

use tracing::info;

#[tokio::main]
async fn main() {
    let banner = assets::Assets::get("banner.txt").unwrap();
    println!(
        "{}",
        std::str::from_utf8(banner.data.as_ref())
            .unwrap()
            .replace("{{version}}", env!("CARGO_PKG_VERSION"))
            .replace("{{commit}}", env!("GIT_COMMIT_ID"))
            .replace("{{build_at}}", env!("BUILD_AT"))
    );

    bootstrap().await;
}

async fn bootstrap() {
    logger::init();
    config::init().await;
    database::init().await;
    container::init().await;
    web::init();

    info!("{:?}", util::jwt::get_secret().await);

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

    axum::serve(listener.unwrap(), web::get_app())
        .await
        .unwrap();
}

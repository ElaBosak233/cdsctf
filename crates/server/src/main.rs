mod logger;
mod migrator;

use std::net::SocketAddr;

use tracing::info;

#[tokio::main]
async fn main() {
    let banner = cds_assets::get("banner.txt").unwrap();
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
    cds_config::init().await;
    cds_queue::init().await;
    cds_cache::init().await;
    cds_db::init().await;

    migrator::run().await;
    let _ = cds_cluster::init().await;
    cds_checker::init().await.unwrap();
    cds_web::init().await;

    let addr = format!(
        "{}:{}",
        cds_config::get_config().server.host,
        cds_config::get_config().server.port
    );
    let listener = tokio::net::TcpListener::bind(&addr).await;

    info!(
        "CdsCTF service has been started at {}. Enjoy your hacking challenges!",
        &addr
    );

    axum::serve(
        listener.unwrap(),
        cds_web::get_app().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
            std::process::exit(0);
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down...");
            std::process::exit(0);
        }
    }
}

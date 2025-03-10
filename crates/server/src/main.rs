mod migrator;

use std::net::SocketAddr;

use anyhow::anyhow;
use chrono::TimeZone;
use tracing::info;

#[tokio::main]
async fn main() {
    bootstrap().await.unwrap_or_else(|err| {
        panic!("Bootstrap error: {}", err);
    });

    let addr = format!(
        "{}:{}",
        cds_config::get_constant().server.host,
        cds_config::get_constant().server.port
    );
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("TcpListener could not bind.");

    info!(
        "CdsCTF service has been started at {}. Enjoy your hacking challenges!",
        &addr
    );

    axum::serve(
        listener,
        cds_web::get_app()
            .to_owned()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown())
    .await
    .expect("Failed to start axum server.");
}

async fn bootstrap() -> Result<(), anyhow::Error> {
    cds_config::init().await?;
    cds_telemetry::init().await?;

    let banner = cds_assets::get("banner.txt").unwrap_or(vec![]);
    println!(
        "{}",
        std::str::from_utf8(&banner)?
            .replace("{{version}}", &cds_config::get_version())
            .replace("{{git_commit}}", &cds_config::get_commit())
            .replace(
                "{{build_at}}",
                chrono::Local
                    .timestamp_opt(cds_config::get_build_timestamp(), 0)
                    .single()
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S UTC %:z")
                    .to_string()
                    .as_str()
            )
    );

    cds_logger::init().await?;

    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| anyhow!("Failed to install `ring` as default crypto provider."))?;

    cds_queue::init().await?;
    cds_cache::init().await?;
    cds_db::init().await?;

    migrator::run().await;

    cds_cluster::init().await?;
    cds_checker::init().await?;
    cds_email::init().await?;
    cds_web::init().await?;

    Ok(())
}

async fn shutdown() {
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
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down...");
        }
    }

    cds_queue::shutdown()
        .await
        .expect("Failed to shutdown queue.");

    cds_telemetry::shutdown()
        .await
        .expect("Failed to shutdown telemetry.");

    std::process::exit(0);
}

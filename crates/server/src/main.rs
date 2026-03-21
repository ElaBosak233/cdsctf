//! CdsCTF **binary entrypoint**: wires configuration, infrastructure clients,
//! HTTP router, and workers.
//!
//! # Lifecycle
//!
//! 1. [`bootstrap`] constructs shared [`AppState`] (database, queue, captcha,
//!    cluster, …).
//! 2. [`cds_worker::init`] starts NATS JetStream consumer tasks.
//! 3. Axum serves HTTP with CORS from config until a shutdown signal arrives.
//! 4. [`shutdown`] drains the message queue and observability exporters before
//!    exit.

use std::{net::SocketAddr, sync::Arc};

use anyhow::anyhow;
use axum::http::HeaderValue;
use cds_web::{router::router, traits::AppState};
use mimalloc::MiMalloc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

/// Global allocator: `mimalloc` is chosen for predictable performance under
/// concurrent load.
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Tokio entrypoint: builds [`AppState`], starts workers, binds Axum, and
/// awaits graceful shutdown.
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Build shared state once; every request handler receives `Arc<AppState>`.
    let state = bootstrap().await?;

    // Reflect allowed browser origins from config into a Tower CORS layer.
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(state.env.server.cors_origins.parse::<HeaderValue>()?);

    // Long-running background jobs (scores, mail, async checks).
    cds_worker::init(&state.db, &state.queue, &state.checker, &state.mailbox).await?;

    let router = router(Arc::clone(&state))
        .await
        .layer(cors)
        .with_state(Arc::clone(&state));

    let addr = format!("{}:{}", state.env.server.host, state.env.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!(
        "CdsCTF service has been started at {}. Enjoy your hacking challenges!",
        &addr
    );

    // `connect_info` enables handlers that need the client socket address (e.g.
    // auditing).
    axum::serve(
        listener,
        router
            .to_owned()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown(Arc::clone(&state)))
    .await?;

    Ok(())
}

/// Loads environment, opens external services, runs migrations, and returns
/// fully populated [`AppState`].
async fn bootstrap() -> Result<Arc<AppState>, anyhow::Error> {
    let env = cds_env::init().await?;
    cds_observe::init(&env).await?;

    let banner = cds_assets::get("banner.txt").unwrap_or_default();

    println!(
        "{}",
        std::str::from_utf8(&banner)?
            .replace("{{version}}", cds_env::get_version())
            .replace("{{git_commit}}", cds_env::get_commit_hash())
            .replace("{{build_at}}", cds_env::get_build_time())
    );

    // `rustls` needs an explicit default crypto provider when multiple backends are
    // linked.
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| anyhow!("Failed to install `ring` as default crypto provider."))?;

    let media = cds_media::init(&env).await?;
    let queue = cds_queue::init(&env).await?;
    let event = cds_event::init(&queue)?;

    let cache = cds_cache::init(&env).await?;
    let db = cds_db::init(&env).await?;

    cds_migrator::run(&db).await?;
    cds_engine::init().await?;
    let checker = cds_checker::init(&media)?;

    let cluster = cds_cluster::init(&env, &checker).await?;

    let mailbox = cds_mailbox::Mailbox::new(db.clone());
    let captcha = cds_captcha::init(&db, &cache)?;

    let state = Arc::from(AppState {
        env,
        event,
        db,
        cache,
        checker,
        captcha,
        cluster,
        media,
        mailbox,
        queue,
    });

    Ok(state)
}

/// Waits for **Ctrl+C** (all platforms) or **SIGTERM** (Unix), then tears down
/// background resources.
async fn shutdown(state: Arc<AppState>) {
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

    state
        .queue
        .shutdown()
        .await
        .expect("Failed to shutdown queue.");

    cds_observe::shutdown(&state.env)
        .await
        .expect("Failed to shutdown observability.");

    std::process::exit(0);
}

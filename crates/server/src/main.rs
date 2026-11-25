//! CdsCTF **binary entrypoint**: wires configuration, infrastructure clients,
//! CQRS score service, HTTP router, and background workers.
//!
//! # Lifecycle
//!
//! 1. [`bootstrap`] constructs shared [`AppState`] (database, cache, queue,
//!    score service, checker, cluster, …).
//! 2. Depending on `env.server.role` (`all` | `api` | `worker`), starts
//!    [`cds_worker::init`] and/or the Axum HTTP listener.
//! 3. [`shutdown`] drains the message queue and observability exporters before
//!    exit.

use std::{net::SocketAddr, sync::Arc};

use anyhow::anyhow;
use axum::http::HeaderValue;
use cds_env::server::Role;
use cds_web::{router::router, traits::AppState};
use mimalloc::MiMalloc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

/// Global allocator: `mimalloc` is chosen for predictable performance under
/// concurrent load.
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Tokio entrypoint: builds [`AppState`], starts configured role(s), and awaits
/// graceful shutdown.
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let state = bootstrap().await?;
    let role = state.env.server.role;

    if matches!(role, Role::All | Role::Worker) {
        cds_worker::init(
            &state.db,
            &state.queue,
            &state.checker,
            &state.score,
            &state.mailbox,
        )
        .await?;
    }

    if matches!(role, Role::All | Role::Api) {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(state.env.server.cors_origins.parse::<HeaderValue>()?);

        let router = router(Arc::clone(&state))
            .await
            .layer(cors)
            .with_state(Arc::clone(&state));

        let addr = format!("{}:{}", state.env.server.host, state.env.server.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        info!(
            role = ?role,
            "CdsCTF service has been started at {}. Enjoy your hacking challenges!",
            &addr
        );

        axum::serve(
            listener,
            router
                .to_owned()
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown(Arc::clone(&state)))
        .await?;
    } else {
        info!(role = ?role, "CdsCTF worker node started");
        shutdown(Arc::clone(&state)).await;
    }

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

    // `rustls` needs an explicit default crypto provider when multiple backends
    // are linked.
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
    let score = cds_score::ScoreService::new(db.clone(), cache.clone(), queue.clone());

    let cluster = cds_cluster::init(&env, &checker).await?;

    let mailbox = cds_mailbox::Mailbox::new(db.clone(), media.clone());
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
        score,
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

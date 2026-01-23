pub mod api;
mod proxy;

use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use axum::{
    Router,
    body::Body,
    http::Request,
    middleware::{from_fn, from_fn_with_state},
    response::Response,
};
use time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::SameSite};
use tower_sessions_redis_store::RedisStore;
use tracing::{Span, debug, debug_span};

use crate::{
    middleware,
    middleware::{error::governor_error, network::GovernorKeyExtractor},
    traits::AppState,
};

pub async fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    // SAFETY: Option<GovernorConfig<_>> could always be unwrapped.
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(state.env.server.burst_restore_rate)
            .burst_size(state.env.server.burst_limit)
            .key_extractor(GovernorKeyExtractor)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            governor_limiter.retain_recent();
        }
    });

    let session_store = RedisStore::new(state.cache.client.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("cds.id")
        .with_secure(false)
        .with_http_only(true)
        .with_path("/")
        .with_same_site(SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(Duration::hours(2)));

    let base = Router::new()
        .nest("/api", api::router().await)
        .route("/healthz", axum::routing::any(|| async { "Ok" }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let ip = crate::util::network::get_client_ip(request)
                        .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
                    debug_span!("http",
                        from = %ip.to_string(),
                        method = %request.method(),
                        uri = %request.uri().path(),
                    )
                })
                .on_request(())
                .on_response(
                    |response: &Response, latency: std::time::Duration, _span: &Span| {
                        debug!("[{}] in {}ms", response.status(), latency.as_millis());
                    },
                ),
        )
        .layer(GovernorLayer::new(governor_conf).error_handler(governor_error))
        .route_layer(from_fn_with_state(state.clone(), middleware::auth::extract))
        .layer(session_layer)
        .layer(from_fn(middleware::network::real_host))
        .layer(from_fn(middleware::network::ip_record));

    let base = state
        .env
        .observe
        .exporter
        .enabled
        .then(|| {
            base.clone()
                .layer(from_fn(middleware::telemetry::track_metrics))
        })
        .unwrap_or(base);

    base.merge(proxy::router(state))
}

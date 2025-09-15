pub mod api;
mod proxy;

use std::{net::IpAddr, sync::Arc};

use axum::{Router, body::Body, http::Request, middleware::from_fn, response::Response};
use time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::SameSite};
use tower_sessions_redis_store::RedisStore;
use tracing::{Span, debug, debug_span};

use crate::{
    middleware,
    middleware::{error::governor_error, network::GovernorKeyExtractor},
};

pub async fn router() -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(cds_env::get_config().server.burst_restore_rate)
            .burst_size(cds_env::get_config().server.burst_limit)
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

    let session_store = RedisStore::new(cds_cache::get_client());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("cds.id")
        .with_secure(false)
        .with_http_only(true)
        .with_path("/")
        .with_same_site(SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));

    let base = Router::new()
        .nest("/api", api::router().await)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let ip = crate::util::network::get_client_ip(request)
                        .unwrap_or(IpAddr::V4("0.0.0.0".parse().unwrap()));
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
        .layer(from_fn(middleware::auth::extract))
        .layer(session_layer)
        .layer(from_fn(middleware::network::real_host))
        .layer(from_fn(middleware::network::ip_record));

    let base = cds_env::get_config()
        .telemetry
        .is_enabled
        .then(|| {
            base.clone()
                .layer(from_fn(middleware::telemetry::track_metrics))
        })
        .unwrap_or(base);

    base.merge(proxy::router())
}

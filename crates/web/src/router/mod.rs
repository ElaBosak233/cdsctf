//! Top-level Axum router: health check, `/api` (documented + protected),
//! reverse proxy, and docs.

pub mod api;
mod healthz;
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
    routing::get,
};
use time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::SameSite};
use tracing::{Span, debug, debug_span, info};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{
    docs::ApiDoc,
    middleware,
    middleware::{error::governor_error, network::GovernorKeyExtractor},
    traits::AppState,
};

/// Builds the full application router for the given [`AppState`].
pub async fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let session_store = cds_cache::session::RedisStore::new(state.cache.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("cds.id")
        .with_secure(false)
        .with_http_only(true)
        .with_path("/")
        .with_same_site(SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(Duration::hours(2)));

    let (documented_axum, paths_openapi) =
        OpenApiRouter::from(Router::new().with_state(state.clone()))
            .nest("/api", api::openapi_documented_under_api(state.clone()))
            .split_for_parts();

    let mut openapi = ApiDoc::openapi();
    openapi.merge(paths_openapi);

    let docs = crate::docs::scalar_router(openapi);

    // All `/api/*` routes (including `/configs`, `/admin`, etc.) come from
    // `openapi_documented_under_api` — do not merge a second copy.
    let mut protected = documented_axum;

    if state.env.server.rate_limit.enabled {
        // SAFETY: Option<GovernorConfig<_>> could always be unwrapped.
        let governor_conf = Arc::new(
            GovernorConfigBuilder::default()
                .per_millisecond(state.env.server.rate_limit.burst_restore_rate)
                .burst_size(state.env.server.rate_limit.burst_size)
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

        info!(
            "Rate limit enabled: Burst size = {}, Restore rate = {} requests/ms",
            state.env.server.rate_limit.burst_size, state.env.server.rate_limit.burst_restore_rate
        );

        protected =
            protected.layer(GovernorLayer::new(governor_conf).error_handler(governor_error));
    }

    protected = protected.route_layer(from_fn_with_state(state.clone(), middleware::auth::extract));

    let mut base = Router::new()
        .route("/healthz", get(healthz::healthz))
        .merge(docs)
        .merge(protected)
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
        .layer(session_layer)
        .layer(from_fn(middleware::network::real_host))
        .layer(from_fn(middleware::network::ip_record));

    if state.env.observe.exporter.enabled {
        base = base.layer(from_fn(middleware::telemetry::track_metrics));
    }

    base.merge(proxy::router(state))
}

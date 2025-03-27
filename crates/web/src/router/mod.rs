pub mod api;
mod proxy;

use std::{net::IpAddr, sync::Arc, time::Duration};

use axum::{Router, body::Body, http::Request, middleware::from_fn, response::Response};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::trace::TraceLayer;
use tracing::{Span, debug, debug_span};

use crate::{
    middleware,
    middleware::{error::governor_error, network::GovernorKeyExtractor},
};

pub async fn router() -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(500)
            .burst_size(32)
            .key_extractor(GovernorKeyExtractor)
            .use_headers()
            .error_handler(governor_error)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            governor_limiter.retain_recent();
        }
    });

    Router::new()
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
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    debug!("[{}] in {}ms", response.status(), latency.as_millis());
                }),
        )
        .layer(GovernorLayer {
            config: governor_conf,
        })
        .layer(from_fn(middleware::auth::extract))
        .layer(from_fn(middleware::network::real_host))
        .layer(from_fn(middleware::network::ip_record))
        .layer(from_fn(middleware::telemetry::track_metrics))
        .merge(proxy::router())
}

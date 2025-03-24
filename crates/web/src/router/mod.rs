pub mod api;
mod proxy;

use std::{net::IpAddr, time::Duration};

use axum::{Router, body::Body, http::Request, middleware::from_fn, response::Response};
use tower_http::trace::TraceLayer;
use tracing::{Span, debug, debug_span};

use crate::middleware;

pub async fn router() -> Router {
    Router::new().merge(
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
            .layer(from_fn(middleware::auth::extract))
            .layer(from_fn(middleware::network::ip_record))
            .layer(from_fn(middleware::telemetry::track_metrics))
            .merge(proxy::router()),
    )
}

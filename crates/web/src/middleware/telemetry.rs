//! HTTP request/response size and concurrency counters for OpenTelemetry
//! metrics.

use axum::{
    body::{Body, HttpBody},
    http::Request,
    middleware::Next,
    response::Response,
};
use cds_observe::exporter::meter::web::{
    get_active_requests, get_request_bytes, get_response_bytes,
};

/// Increments in-flight gauge, records approximate byte sizes, then decrements
/// after the inner service responds.
pub async fn track_metrics(req: Request<Body>, next: Next) -> Response {
    get_active_requests().add(1, &[]);

    let request_size = req.size_hint().lower();
    get_request_bytes().add(request_size, &[]);

    let response = next.run(req).await;

    get_response_bytes().add(response.size_hint().lower(), &[]);
    get_active_requests().add(-1, &[]);

    response
}

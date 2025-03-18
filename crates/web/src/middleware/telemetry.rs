use axum::body::{Body, HttpBody};
use axum::response::Response;
use axum::{
    http::Request,
    middleware::Next,
};
use cds_telemetry::meter::web::{get_active_requests, get_request_bytes, get_response_bytes};

pub async fn track_metrics(req: Request<Body>, next: Next) -> Response {
    get_active_requests().add(1, &[]);

    let request_size = req.size_hint().lower();
    get_request_bytes().add(request_size, &[]);

    let response = next.run(req).await;

    get_response_bytes().add(response.size_hint().lower(), &[]);
    get_active_requests().add(-1, &[]);

    response
}
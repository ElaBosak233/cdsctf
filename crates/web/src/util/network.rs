//! Web utility — `network` (shared HTTP helpers).

use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Request},
    http::HeaderMap,
};

const X_REAL_IP: &str = "x-real-ip";
const X_FORWARDED_FOR: &str = "x-forwarded-for";

/// Parses `X-Forwarded-For` when present.
fn maybe_x_forwarded_for(headers: &HeaderMap) -> Option<IpAddr> {
    headers
        .get(X_FORWARDED_FOR)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.split(",").find_map(|s| s.trim().parse::<IpAddr>().ok()))
}

/// Parses `X-Real-IP` when present.
fn maybe_x_real_ip(headers: &HeaderMap) -> Option<IpAddr> {
    headers
        .get(X_REAL_IP)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
}

/// Falls back to Axum connect info for the client IP.
fn maybe_connect_info<B>(req: &Request<B>) -> Option<IpAddr> {
    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}

/// Returns client ip.

pub fn get_client_ip<B>(request: &Request<B>) -> Option<IpAddr> {
    let headers = request.headers();
    maybe_x_forwarded_for(headers)
        .or_else(|| maybe_x_real_ip(headers))
        .or_else(|| maybe_connect_info(request))
}

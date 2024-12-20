mod worker;

use axum::{Router, response::IntoResponse};
use cds_metric::METRICS_REGISTRY;
use prometheus::{Encoder, TextEncoder};

pub async fn router() -> Router {
    worker::init().await;

    Router::new().route("/", axum::routing::get(metrics))
}

pub async fn metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = METRICS_REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder
        .encode(&metric_families, &mut buffer)
        .expect("encode metrics");
    String::from_utf8(buffer)
        .expect("metrics buffer is UTF-8")
        .into_response()
}

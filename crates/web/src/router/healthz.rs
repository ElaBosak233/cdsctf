//! Plain Axum route for `/healthz`; not included in the OpenAPI document.

/// Liveness probe body; returns plain text `Ok`.
pub async fn healthz() -> &'static str {
    "Ok"
}

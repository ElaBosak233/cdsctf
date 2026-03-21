//! OpenAPI metadata and Scalar API reference wiring.
//!
//! Only the `/api` subtree built by
//! [`crate::router::api::openapi_documented_under_api`] is aggregated from
//! nested [`utoipa_axum::router::OpenApiRouter`] layers and merged into
//! [`ApiDoc`] via [`utoipa::OpenApi::merge`]. Schemas such as
//! [`crate::traits::ErrorResponse`], [`crate::traits::EmptyJson`], and
//! route-specific bodies are contributed by those operations;
//! this base document intentionally omits `components.schemas` to avoid
//! duplicates after merge.

use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::traits::AppState;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "CdsCTF API",
        description = "HTTP API for the CdsCTF platform.",
        version = env!("CARGO_PKG_VERSION")
    ),
    tags(
        (name = "system", description = "Service metadata and probes"),
        (name = "config", description = "Public configuration"),
    ),
)]
pub struct ApiDoc;

/// Serves interactive API docs at `/docs` for the given merged OpenAPI
/// document.
pub fn scalar_router(openapi: utoipa::openapi::OpenApi) -> axum::Router<Arc<AppState>> {
    Router::from(Scalar::with_url("/docs", openapi))
}

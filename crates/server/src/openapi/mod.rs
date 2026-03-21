//! OpenAPI 元数据与 Scalar API Reference。
//!
//! 路径由各层 [`utoipa_axum::router::OpenApiRouter`] 汇总后，与 [`ApiDoc`] 的 `info` / `components` 做 [`OpenApi::merge`]。

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
        version = env!("CARGO_PKG_VERSION"),
        license(name = "MIT OR Apache-2.0")
    ),
    tags(
        (name = "system", description = "Service metadata and probes"),
        (name = "config", description = "Public configuration"),
    ),
    components(schemas(
        crate::router::api::ApiIndexResponse,
        crate::router::api::config::Version,
        crate::router::api::config::ConfigResponse,
    ))
)]
pub struct ApiDoc;

pub fn scalar_router(openapi: utoipa::openapi::OpenApi) -> axum::Router<Arc<AppState>> {
    Router::from(Scalar::with_url("/docs", openapi))
}

//! HTTP handlers for `proxy` within the `router` API segment.

use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::{Request, header::CACHE_CONTROL},
    middleware::Next,
    response::Response,
};
use tower_http::services::ServeFile;

use crate::traits::AppState;

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .fallback_service(
            tower_http::services::ServeDir::new(&state.env.server.frontend)
                .precompressed_gzip()
                .not_found_service(ServeFile::new(format!(
                    "{}/index.html",
                    state.env.server.frontend
                ))),
        )
        .layer(axum::middleware::from_fn(cache_control))
}

/// Applies cache policy to files served by the frontend proxy.
///
/// Vite assets contain a content hash and can be cached indefinitely. The SPA
/// entry point and runtime-loaded locale files keep stable URLs, so they must
/// be revalidated on every deployment.
async fn cache_control(request: Request<Body>, next: Next) -> Response {
    let path = request.uri().path().to_owned();
    let mut response = next.run(request).await;

    if let Some(value) = cache_control_value(&path) {
        response.headers_mut().insert(CACHE_CONTROL, value);
    }

    response
}

fn cache_control_value(path: &str) -> Option<axum::http::HeaderValue> {
    let value = if path == "/" || path.ends_with("/index.html") {
        "no-store, max-age=0"
    } else if path.starts_with("/locales/") {
        "no-store, max-age=0"
    } else if path.starts_with("/assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache, max-age=0, must-revalidate"
    };

    Some(axum::http::HeaderValue::from_static(value))
}

#[cfg(test)]
mod tests {
    use super::cache_control_value;

    #[test]
    fn entry_and_locale_files_are_not_stored() {
        assert_eq!(cache_control_value("/").unwrap(), "no-store, max-age=0");
        assert_eq!(
            cache_control_value("/locales/zh-CN/common.yaml").unwrap(),
            "no-store, max-age=0"
        );
    }

    #[test]
    fn hashed_assets_are_immutable() {
        assert_eq!(
            cache_control_value("/assets/index-C5x9.js").unwrap(),
            "public, max-age=31536000, immutable"
        );
    }

    #[test]
    fn other_public_files_must_revalidate() {
        assert_eq!(
            cache_control_value("/logo.svg").unwrap(),
            "no-cache, max-age=0, must-revalidate"
        );
    }
}

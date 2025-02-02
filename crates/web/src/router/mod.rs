pub mod api;

use axum::{Router, middleware::from_fn};
use tower_http::trace::TraceLayer;

use crate::middleware;

pub async fn router() -> Router {
    Router::new().merge(
        Router::new()
            .nest("/api", api::router().await)
            .layer(TraceLayer::new_for_http())
            .layer(from_fn(middleware::auth))
            .layer(from_fn(middleware::network::ip_record)),
    )
}

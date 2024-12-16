pub mod api;
pub mod metric;

use axum::{middleware::from_fn, Router};
use tower_http::trace::TraceLayer;

use crate::web::middleware;

pub async fn router() -> Router {
    Router::new()
        .merge(
            Router::new()
                .nest("/api", api::router().await)
                .layer(TraceLayer::new_for_http())
                .layer(from_fn(middleware::auth))
                .layer(from_fn(middleware::network)),
        )
        .nest("/metrics", metric::router().await)
}

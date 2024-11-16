pub mod api;
pub mod metric;

use axum::{middleware::from_fn, response::IntoResponse, Router};
use tower_http::trace::TraceLayer;

use crate::web::middleware;

pub async fn router() -> Router {
    Router::new()
        .merge(
            Router::new()
                .nest("/api", api::router().await)
                .layer(from_fn(middleware::auth::jwt))
                .layer(TraceLayer::new_for_http()),
        )
        .nest("/metrics", metric::router().await)
}

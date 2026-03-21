use std::sync::Arc;

use axum::Router;
use tower_http::services::ServeFile;

use crate::traits::AppState;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().fallback_service(
        tower_http::services::ServeDir::new(&state.env.server.frontend)
            .precompressed_gzip()
            .not_found_service(ServeFile::new(format!(
                "{}/index.html",
                state.env.server.frontend
            ))),
    )
}

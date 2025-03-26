mod attachment;

use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/attachment", attachment::router())
}

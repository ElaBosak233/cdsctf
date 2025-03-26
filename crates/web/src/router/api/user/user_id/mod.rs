use axum::Router;

mod avatar;

pub fn router() -> Router {
    Router::new().nest("/avatar", avatar::router())
}

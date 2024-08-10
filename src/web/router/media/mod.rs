use axum::{routing::get, Router};

use crate::web::handler;

pub fn router() -> Router {
    return Router::new().route("/*path", get(handler::media::get_file));
}

use axum::{routing::get, Router};

use crate::web::controller;

pub fn router() -> Router {
    return Router::new().route("/*path", get(controller::media::get_file));
}

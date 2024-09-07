pub mod challenge;
pub mod config;
pub mod game;
pub mod media;
pub mod pod;
pub mod proxy;
pub mod submission;
pub mod team;
pub mod user;

use axum::http::StatusCode;
use axum::{Json, Router};
use serde_json::json;

pub async fn router() -> Router {
    return Router::new()
        .route(
            "/",
            axum::routing::any(|| async {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "code": StatusCode::OK.as_u16(),
                        "msg": format!("{:?}", "This is the heart of Cloudsdale!")
                    })),
                );
            }),
        )
        .nest("/configs", config::router())
        .nest("/media", media::router())
        .nest("/proxies", proxy::router())
        .nest("/users", user::router())
        .nest("/teams", team::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router().await)
        .nest("/pods", pod::router().await)
        .nest("/submissions", submission::router().await);
}

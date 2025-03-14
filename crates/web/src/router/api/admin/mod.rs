mod challenge;
mod config;
mod env;
mod game;
mod submission;
mod user;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/users", user::router())
        .nest("/challenges", challenge::router())
        .nest("/games", game::router())
        .nest("/envs", env::router())
        .nest("/submissions", submission::router())
        .nest("/configs", config::router())
}

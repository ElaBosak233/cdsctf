//! HTTP routing for `admin` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge` submodule (see sibling `*.rs` files).
mod challenge;

/// Defines the `config` submodule (see sibling `*.rs` files).
mod config;

/// Defines the `game` submodule (see sibling `*.rs` files).
mod game;

/// Defines the `instance` submodule (see sibling `*.rs` files).
mod instance;

/// Defines the `idp` submodule (see sibling `*.rs` files).
mod idp;

/// Defines the `submission` submodule (see sibling `*.rs` files).
mod submission;

/// Defines the `user` submodule (see sibling `*.rs` files).
mod user;

use std::sync::Arc;

use axum::Router;
use utoipa_axum::router::OpenApiRouter;

use crate::traits::AppState;

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .nest("/instances", instance::router(state.clone()))
        .nest("/submissions", submission::router(state.clone()))
        .nest("/users", user::router(state.clone()))
        .nest("/challenges", challenge::router(state.clone()))
        .nest("/games", game::router(state.clone()))
        .nest("/idps", idp::router(state.clone()))
        .nest("/configs", config::router(state.clone()))
}

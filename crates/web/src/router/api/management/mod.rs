//! HTTP routing for `management` — Axum router wiring and OpenAPI route
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

use axum::{Router, middleware::from_fn};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    middleware::auth::{
        require_management_challenge, require_management_game, require_management_idp,
        require_management_user,
    },
    traits::AppState,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .nest(
            "/instances",
            instance::router(state.clone()).route_layer(from_fn(require_management_challenge)),
        )
        .nest(
            "/submissions",
            submission::router(state.clone()).route_layer(from_fn(require_management_game)),
        )
        .nest(
            "/users",
            user::router(state.clone()).route_layer(from_fn(require_management_user)),
        )
        .nest(
            "/challenges",
            challenge::router(state.clone()).route_layer(from_fn(require_management_challenge)),
        )
        .nest(
            "/games",
            game::router(state.clone()).route_layer(from_fn(require_management_game)),
        )
        .nest(
            "/idps",
            idp::router(state.clone()).route_layer(from_fn(require_management_idp)),
        )
        .nest("/configs", config::router(state.clone()))
}

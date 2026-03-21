//! HTTP layer for the CdsCTF platform: Axum routers, OpenAPI documentation,
//! middleware, models, utilities.
//!
//! Entry point for route composition is [`router::router`]; shared dependencies
//! live in [`traits::AppState`].

/// Defines the `docs` submodule (see sibling `*.rs` files).
pub mod docs;

/// Defines the `extract` submodule (see sibling `*.rs` files).
pub mod extract;

/// Defines the `middleware` submodule (see sibling `*.rs` files).
pub mod middleware;

/// Defines the `model` submodule (see sibling `*.rs` files).
pub mod model;

/// Defines the `router` submodule (see sibling `*.rs` files).
pub mod router;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Defines the `util` submodule (see sibling `*.rs` files).
pub mod util;

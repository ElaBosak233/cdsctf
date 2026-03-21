//! Axum middleware layers: authentication/authorization, client IP + host
//! normalization, rate-limit error mapping, request metrics, and shared error
//! helpers.

/// Defines the `auth` submodule (see sibling `*.rs` files).
pub mod auth;

/// Defines the `error` submodule (see sibling `*.rs` files).
pub mod error;

/// Defines the `network` submodule (see sibling `*.rs` files).
pub mod network;

/// Defines the `telemetry` submodule (see sibling `*.rs` files).
pub mod telemetry;

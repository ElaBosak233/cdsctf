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

/// A positive #ID selects one record; other nonempty input searches display
/// text.
fn parse_list_search(query: Option<&str>) -> (Option<i64>, Option<String>) {
    let keyword = query.map(str::trim).filter(|q| !q.is_empty());
    let search_id = keyword
        .and_then(|q| q.strip_prefix('#'))
        .filter(|id| id.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|id| id.parse::<i64>().ok())
        .filter(|&id| id > 0);
    let keyword = keyword.filter(|_| search_id.is_none()).map(str::to_owned);
    (search_id, keyword)
}

#[cfg(test)]
mod list_search_tests {
    use super::parse_list_search;

    #[test]
    fn parses_ids_and_keeps_other_input_literal() {
        assert_eq!(parse_list_search(None), (None, None));
        assert_eq!(parse_list_search(Some("  ")), (None, None));
        assert_eq!(parse_list_search(Some(" #0012 ")), (Some(12), None));
        for text in [
            "12",
            "#0",
            "#+12",
            "#web",
            "#99999999999999999999",
            "100%_foo",
        ] {
            assert_eq!(parse_list_search(Some(text)), (None, Some(text.to_owned())));
        }
    }
}

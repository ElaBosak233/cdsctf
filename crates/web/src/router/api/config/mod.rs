//! HTTP routing for `config` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `captcha` submodule (see sibling `*.rs` files).
mod captcha;

/// Defines the `logo` submodule (see sibling `*.rs` files).
mod logo;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::PublicConfig;
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::{AppState, WebError};

/// Nests under [`OpenApiRouter::nest("/configs", ...)`]; paths are relative to
/// `/configs`.
pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_config).with_state(state.clone()))
        .routes(routes!(crate::router::api::admin::config::update_config).with_state(state.clone()))
        .routes(
            routes!(crate::router::api::admin::config::get_statistics).with_state(state.clone()),
        )
        .routes(routes!(get_version).with_state(state.clone()))
        .nest(
            "/logo",
            OpenApiRouter::from(Router::new().with_state(state.clone()))
                .routes(routes!(logo::get_logo).with_state(state.clone()))
                .routes(
                    routes!(crate::router::api::admin::config::logo::save_logo)
                        .with_state(state.clone())
                        .layer(axum::extract::DefaultBodyLimit::max(5 * 1024 * 1024)),
                )
                .routes(
                    routes!(crate::router::api::admin::config::logo::delete_logo)
                        .with_state(state.clone()),
                ),
        )
        .nest(
            "/captcha",
            OpenApiRouter::from(Router::new().with_state(state.clone()))
                .routes(routes!(captcha::generate_captcha).with_state(state.clone())),
        )
        .nest(
            "/mailbox",
            crate::router::api::admin::config::email::router(state.clone()),
        )
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ConfigResponse {
    pub config: PublicConfig,
}

/// Returns config.
#[utoipa::path(
    get,
    path = "/",
    tag = "config",
    responses(
        (status = 200, description = "Desensitized site configuration", body = ConfigResponse),
        (status = 401, description = "Session error", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_config"))]
pub async fn get_config(
    State(s): State<Arc<AppState>>,
    crate::extract::Extension(principal): crate::extract::Extension<crate::traits::AuthPrincipal>,
) -> Result<Json<serde_json::Value>, WebError> {
    let config = cds_db::get_config(&s.db.conn).await;
    if principal
        .operator
        .is_some_and(|user| user.group >= cds_db::user::Group::Admin)
    {
        return Ok(Json(serde_json::json!({ "config": config })));
    }
    Ok(Json(serde_json::json!({
        "config": PublicConfig::from(&config)
    })))
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Version {
    pub tag: String,
    pub commit: String,
}

/// Returns version.
#[utoipa::path(
    get,
    path = "/version",
    tag = "config",
    responses(
        (status = 200, description = "Build tag and git commit", body = Version),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_version"))]
pub async fn get_version() -> Result<Json<Version>, WebError> {
    Ok(Json(Version {
        tag: cds_env::get_version().to_owned(),
        commit: cds_env::get_commit_hash().to_owned(),
    }))
}

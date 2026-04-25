//! HTTP routing for `config` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `captcha` submodule (see sibling `*.rs` files).
mod captcha;

/// Defines the `logo` submodule (see sibling `*.rs` files).
mod logo;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::Config;
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
        .routes(routes!(get_version).with_state(state.clone()))
        .nest(
            "/logo",
            OpenApiRouter::from(Router::new().with_state(state.clone()))
                .routes(routes!(logo::get_logo).with_state(state.clone())),
        )
        .nest(
            "/captcha",
            OpenApiRouter::from(Router::new().with_state(state.clone()))
                .routes(routes!(captcha::generate_captcha).with_state(state.clone())),
        )
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ConfigResponse {
    pub config: Config,
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
pub async fn get_config(State(s): State<Arc<AppState>>) -> Result<Json<ConfigResponse>, WebError> {
    Ok(Json(ConfigResponse {
        config: cds_db::get_config(&s.db.conn).await.desensitize(),
    }))
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

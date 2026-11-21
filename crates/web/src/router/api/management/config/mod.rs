//! HTTP routing for `config` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `logo` submodule (see sibling `*.rs` files).
mod logo;

use std::sync::Arc;

use axum::{Json, Router, extract::State, middleware::from_fn};
use cds_db::config::{Config, auth, captcha, email, meta};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Json as ReqJson,
    middleware::auth::require_management_config,
    traits::{AppState, WebError},
    util::email::normalize_whitelist,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    let protected = OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_config).with_state(state.clone()))
        .routes(routes!(update_config).with_state(state.clone()))
        .nest("/logo", logo::router(state.clone()))
        .route_layer(from_fn(require_management_config));

    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_statistics).with_state(state))
        .merge(protected)
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminConfigResponse {
    pub config: AdminConfig,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminConfig {
    meta: meta::Config,
    auth: auth::Config,
    email: AdminEmailConfig,
    captcha: captcha::Config,
    logo_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminEmailConfig {
    enabled: bool,
    host: String,
    port: u16,
    tls: email::Tls,
    username: String,
    whitelist: Vec<String>,
}

impl From<Config> for AdminConfig {
    fn from(config: Config) -> Self {
        Self {
            meta: config.meta,
            auth: config.auth,
            email: AdminEmailConfig {
                enabled: config.email.enabled,
                host: config.email.host,
                port: config.email.port,
                tls: config.email.tls,
                username: config.email.username,
                whitelist: config.email.whitelist,
            },
            captcha: config.captcha,
            logo_hash: config.logo_hash,
        }
    }
}

/// Returns config.
#[utoipa::path(
    get,
    path = "/",
    tag = "management-config",
    responses(
        (status = 200, description = "Administrative config", body = AdminConfigResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_config"))]
pub async fn get_config(
    State(s): State<Arc<AppState>>,
) -> Result<Json<AdminConfigResponse>, WebError> {
    Ok(Json(AdminConfigResponse {
        config: cds_db::get_config(&s.db.conn).await.into(),
    }))
}

/// Updates config.
#[utoipa::path(
    patch,
    path = "/",
    tag = "management-config",
    request_body = Config,
    responses(
        (status = 200, description = "Saved config", body = AdminConfigResponse),
        (status = 400, description = "Invalid configuration", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_config"))]
pub async fn update_config(
    State(s): State<Arc<AppState>>,
    ReqJson(mut body): ReqJson<Config>,
) -> Result<Json<AdminConfigResponse>, WebError> {
    normalize_whitelist(&mut body.email.whitelist)?;
    let config = cds_db::config::save(&s.db.conn, body).await?;
    Ok(Json(AdminConfigResponse {
        config: config.into(),
    }))
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Statistics {
    pub users: u64,
    pub games: u64,
    pub challenges: ChallengeStatistics,
    pub submissions: SubmissionStatistics,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChallengeStatistics {
    pub total: u64,
    pub in_game: u64,
    pub standalone: u64,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SubmissionStatistics {
    pub total: u64,
    pub solved: u64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct StatisticsResponse {
    pub statistics: Statistics,
}

/// Returns statistics.
#[utoipa::path(
    get,
    path = "/statistics",
    tag = "management-config",
    responses(
        (status = 200, description = "Counts", body = StatisticsResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_statistics"))]
pub async fn get_statistics(
    State(s): State<Arc<AppState>>,
) -> Result<Json<StatisticsResponse>, WebError> {
    let challenges_total = cds_db::challenge::count(&s.db.conn).await?;
    let challenges_in_game =
        cds_db::game_challenge::count_distinct_challenges(&s.db.conn).await?;

    Ok(Json(StatisticsResponse {
        statistics: Statistics {
            users: cds_db::user::count(&s.db.conn).await?,
            games: cds_db::game::count(&s.db.conn).await?,
            challenges: ChallengeStatistics {
                total: challenges_total,
                in_game: challenges_in_game,
                standalone: challenges_total.saturating_sub(challenges_in_game),
            },
            submissions: SubmissionStatistics {
                total: cds_db::submission::count(&s.db.conn).await?,
                solved: cds_db::submission::count_correct(&s.db.conn).await?,
            },
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_config_omits_smtp_password_and_can_be_saved_without_it() {
        let mut config = Config::default();
        config.email.host = "smtp.example.org".to_owned();
        config.email.password = "stored-secret".to_owned();

        let response = AdminConfigResponse {
            config: config.into(),
        };
        let json = serde_json::to_value(response).unwrap();
        assert!(json["config"]["email"].get("password").is_none());

        let update: Config = serde_json::from_value(json["config"].clone()).unwrap();
        assert_eq!(update.email.host, "smtp.example.org");
        assert!(update.email.password.is_empty());
    }
}

mod email;
mod logo;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::Config;
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Json as ReqJson,
    traits::{AppState, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_config).with_state(state.clone()))
        .routes(routes!(update_config).with_state(state.clone()))
        .routes(routes!(get_statistics).with_state(state.clone()))
        .nest("/logo", logo::router(state.clone()))
        .nest("/mailbox", email::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminConfigResponse {
    pub config: Config,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-config",
    responses(
        (status = 200, description = "Full config", body = AdminConfigResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_config(
    State(s): State<Arc<AppState>>,
) -> Result<Json<AdminConfigResponse>, WebError> {
    Ok(Json(AdminConfigResponse {
        config: cds_db::get_config(&s.db.conn).await,
    }))
}

#[utoipa::path(
    put,
    path = "/",
    tag = "admin-config",
    request_body = Config,
    responses(
        (status = 200, description = "Saved config", body = AdminConfigResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn update_config(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<Config>,
) -> Result<Json<AdminConfigResponse>, WebError> {
    let config = cds_db::config::save(&s.db.conn, body).await?;
    Ok(Json(AdminConfigResponse { config }))
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

#[utoipa::path(
    get,
    path = "/statistics",
    tag = "admin-config",
    responses(
        (status = 200, description = "Counts", body = StatisticsResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_statistics(
    State(s): State<Arc<AppState>>,
) -> Result<Json<StatisticsResponse>, WebError> {
    Ok(Json(StatisticsResponse {
        statistics: Statistics {
            users: cds_db::user::count(&s.db.conn).await?,
            games: cds_db::game::count(&s.db.conn).await?,
            challenges: ChallengeStatistics {
                total: cds_db::challenge::count(&s.db.conn).await?,
                in_game: cds_db::game_challenge::count(&s.db.conn).await?,
            },
            submissions: SubmissionStatistics {
                total: cds_db::submission::count(&s.db.conn).await?,
                solved: cds_db::submission::count_correct(&s.db.conn).await?,
            },
        },
    }))
}

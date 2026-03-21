mod captcha;
mod logo;

use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::Config;
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::{AppState, WebError, WebResponse};

/// Logo、captcha 等未单独写 `#[utoipa::path]` 的子路由，挂在 `/configs` 下与 OpenAPI 子树合并。
pub fn router_logo_and_captcha() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/logo", logo::router())
        .nest("/captcha", captcha::router())
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .merge(router_logo_and_captcha())
        .route("/version", axum::routing::get(get_version))
}

/// 汇总到上层 [`OpenApiRouter::nest("/configs", ...)`]；路径相对于 `/configs`。
pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_config).with_state(state.clone()))
        .routes(routes!(get_version).with_state(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "config",
    responses(
        (status = 200, description = "Desensitized site configuration (`WebResponse` JSON)", body = serde_json::Value),
        (status = 401, description = "Session error"),
        (status = 500, description = "Server error"),
    )
)]
pub async fn get_config(State(s): State<Arc<AppState>>) -> Result<WebResponse<Config>, WebError> {
    Ok(WebResponse {
        data: Some(cds_db::get_config(&s.db.conn).await.desensitize()),
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Version {
    pub tag: String,
    pub commit: String,
}

#[utoipa::path(
    get,
    path = "/version",
    tag = "config",
    responses(
        (status = 200, description = "Build tag and git commit (`WebResponse` JSON)", body = serde_json::Value),
        (status = 500, description = "Server error"),
    )
)]
pub async fn get_version() -> Result<WebResponse<Version>, WebError> {
    Ok(WebResponse {
        data: Some(Version {
            tag: cds_env::get_version().to_owned(),
            commit: cds_env::get_commit_hash().to_owned(),
        }),
        ..Default::default()
    })
}

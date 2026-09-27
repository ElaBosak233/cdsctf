//! Public IdP routes: IdP list, Lua-backed login, bind, and unbind.

/// Defines the `idp_id` submodule (see sibling `*.rs` files).
pub(crate) mod idp_id;

use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::State};
use cds_db::{IdpSummary, IdpView, user::Group};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_idps).with_state(state.clone()))
        .routes(routes!(crate::router::api::admin::idp::create_idp).with_state(state.clone()))
        .routes(routes!(crate::router::api::admin::idp::update_idp).with_state(state.clone()))
        .routes(routes!(crate::router::api::admin::idp::delete_idp).with_state(state.clone()))
        .routes(routes!(crate::router::api::admin::idp::lint_idp_script).with_state(state.clone()))
        .nest("/{idp_id}", idp_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpsResponse {
    pub idps: Vec<IdpSummary>,
}

#[derive(Clone, Debug, Deserialize, utoipa::ToSchema)]
pub struct IdpAuthRequest {
    #[serde(default)]
    pub params: HashMap<String, String>,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "idp",
    responses((status = 200, description = "Enabled IdPs", body = IdpsResponse))
)]
#[tracing::instrument(skip_all, fields(handler = "list_idps"))]
pub async fn list_idps(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
) -> Result<Json<serde_json::Value>, WebError> {
    if principal
        .operator
        .is_some_and(|user| user.group >= Group::Admin)
    {
        let idps = cds_db::idp::find_idps::<IdpView>(&s.db.conn).await?;
        return Ok(Json(serde_json::json!({ "idps": idps })));
    }
    let idps = cds_db::idp::find_public_idps::<IdpSummary>(&s.db.conn).await?;
    Ok(Json(serde_json::json!({ "idps": idps })))
}

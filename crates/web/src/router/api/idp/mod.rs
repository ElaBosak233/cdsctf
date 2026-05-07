//! Public IdP routes: IdP list, Rune-backed login, bind, and unbind.

/// Defines the `idp_id` submodule (see sibling `*.rs` files).
mod idp_id;

use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::State};
use cds_db::Idp;
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::{AppState, WebError};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_idps).with_state(state.clone()))
        .nest("/{idp_id}", idp_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpsResponse {
    pub idps: Vec<Idp>,
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
pub async fn list_idps(State(s): State<Arc<AppState>>) -> Result<Json<IdpsResponse>, WebError> {
    let idps = cds_db::idp::find_public_idps::<Idp>(&s.db.conn)
        .await?
        .into_iter()
        .map(Idp::desensitize)
        .collect();
    Ok(Json(IdpsResponse { idps }))
}

//! Public IdP routes: IdP list, Rune-backed login, bind, and unbind.

/// Defines the `idp_id` submodule (see sibling `*.rs` files).
mod idp_id;

use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::State};
use cds_db::{Email, Idp, User, UserIdp, sea_orm::ActiveValue::{NotSet, Set}};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

pub(crate) async fn enabled_idp(s: &AppState, idp_id: i64) -> Result<Idp, WebError> {
    let idp = cds_db::idp::find_idp_by_id::<Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;
    if !idp.enabled {
        return Err(WebError::NotFound(json!("idp_not_found")));
    }
    Ok(idp)
}

pub(crate) async fn user_map(
    s: &AppState,
    user: &User,
) -> Result<HashMap<String, String>, WebError> {
    let mut map = HashMap::from([
        ("id".to_string(), user.id.to_string()),
        ("username".to_string(), user.username.clone()),
        ("name".to_string(), user.name.clone()),
        (
            "group".to_string(),
            format!("{:?}", user.group).to_lowercase(),
        ),
    ]);
    if let Some(email) = cds_db::email::find_by_user_id::<Email>(&s.db.conn, user.id)
        .await?
        .into_iter()
        .find(|email| email.verified)
    {
        map.insert("email".to_string(), email.email);
    }
    Ok(map)
}

pub(crate) async fn create_user_idp(
    s: &AppState,
    idp: &Idp,
    user_id: i64,
    payload: &cds_idp::IdentityPayload,
) -> Result<UserIdp, WebError> {
    Ok(cds_db::user_idp::create_user_idp::<UserIdp>(
        &s.db.conn,
        cds_db::user_idp::UserIdpActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            idp_id: Set(idp.id),
            auth_key: Set(payload.auth_key.clone()),
            data: Set(Some(json!(payload.data))),
            ..Default::default()
        },
    )
    .await?)
}

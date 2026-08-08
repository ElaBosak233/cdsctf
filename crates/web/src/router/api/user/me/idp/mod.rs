//! Current-user IdP binding routes.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
};
use cds_db::{UserIdpSource, UserIdpSummary, user_idp::UserIdpModel};
use serde::Serialize;
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_my_idps).with_state(state.clone()))
        .routes(routes!(unbind_my_idp).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct UserIdpsResponse {
    pub idps: Vec<UserIdpSummary>,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    responses(
        (status = 200, description = "Bound IdPs", body = UserIdpsResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_my_idps"))]
pub async fn list_my_idps(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<UserIdpsResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let idps =
        cds_db::user_idp::find_user_idps_by_user::<UserIdpSummary>(&s.db.conn, operator.id).await?;
    Ok(Json(UserIdpsResponse { idps }))
}

#[utoipa::path(
    delete,
    path = "/{user_idp_id}",
    tag = "user",
    params(("user_idp_id" = i64, Path, description = "User IdP binding id")),
    responses(
        (status = 200, description = "IdP unbound", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "unbind_my_idp"))]
pub async fn unbind_my_idp(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(user_idp_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let identity = cds_db::user_idp::find_user_idp_by_id_and_user::<UserIdpModel>(
        &s.db.conn,
        user_idp_id,
        operator.id,
    )
    .await?
    .ok_or(WebError::NotFound(json!("user_idp_not_found")))?;
    ensure_unbindable(&identity.source)?;
    cds_db::user_idp::delete_user_idp(&s.db.conn, operator.id, user_idp_id).await?;
    Ok(Json(EmptyJson::default()))
}

fn ensure_unbindable(source: &UserIdpSource) -> Result<(), WebError> {
    match source {
        UserIdpSource::Registration => Err(WebError::Conflict(json!(
            "registration_idp_cannot_be_unbound"
        ))),
        UserIdpSource::Binding => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use cds_db::UserIdpSource;

    use super::ensure_unbindable;
    use crate::traits::WebError;

    #[test]
    fn registration_source_is_not_user_unbindable() {
        assert!(matches!(
            ensure_unbindable(&UserIdpSource::Registration),
            Err(WebError::Conflict(_))
        ));
        assert!(ensure_unbindable(&UserIdpSource::Binding).is_ok());
    }
}

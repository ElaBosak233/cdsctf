//! HTTP routing for `challenge_id` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `attachment` submodule (see sibling `*.rs` files).
mod attachment;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::Challenge;
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenge).with_state(state.clone()))
        .nest("/attachments", attachment::router(state.clone()))
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ChallengeDetailResponse {
    pub challenge: Challenge,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Challenge", body = ChallengeDetailResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns challenge.
pub async fn get_challenge(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<ChallengeDetailResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .desensitize();

    if !cds_db::util::can_user_access_challenge(&s.db.conn, operator.id, challenge.id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    Ok(Json(ChallengeDetailResponse {
        challenge: challenge.desensitize(),
    }))
}

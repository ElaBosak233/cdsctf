//! Writeup management for a challenge.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{ChallengeDetail, sea_orm::{Set, Unchanged}};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa_axum::{router::{OpenApiRouter, UtoipaMethodRouterExt}, routes};

use crate::{
    extract::{Extension, Path, VJson},
    traits::{AppState, AuthPrincipal, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_writeup).with_state(state))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct UpdateWriteupRequest {
    pub writeup: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ChallengeWriteupResponse {
    pub challenge: ChallengeDetail,
}

/// Updates a challenge writeup. Only a global administrator may mutate it.
#[utoipa::path(
    put,
    path = "/",
    tag = "challenge",
    params(("challenge_id" = i64, Path, description = "Challenge id")),
    request_body = UpdateWriteupRequest,
    responses(
        (status = 200, description = "Updated writeup", body = ChallengeWriteupResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_writeup"))]
pub async fn update_writeup(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateWriteupRequest>,
) -> Result<Json<ChallengeWriteupResponse>, WebError> {
    crate::middleware::auth::require_admin(&principal)?;
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;
    let challenge = cds_db::challenge::update(
        &s.db.conn,
        cds_db::challenge::ActiveModel {
            id: Unchanged(challenge.id),
            writeup: Set(Some(body.writeup)),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(ChallengeWriteupResponse { challenge }))
}

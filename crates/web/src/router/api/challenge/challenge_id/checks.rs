//! Synchronous checker previews for a challenge.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::submission::Status;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa_axum::{router::{OpenApiRouter, UtoipaMethodRouterExt}, routes};

use crate::{
    extract::{Extension, Path, VJson},
    traits::{AppState, AuthPrincipal, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(create_check).with_state(state))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct CreateCheckRequest {
    pub content: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CreateCheckResponse {
    pub status: Status,
}

pub(crate) async fn run_check(
    s: &AppState,
    operator_id: i64,
    challenge_id: i64,
    content: &str,
) -> Result<CreateCheckResponse, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;
    let result = s.checker.check(&challenge, operator_id, content).await;
    let status = match result {
        Ok(cds_checker::Status::Correct) => Status::Correct,
        Ok(cds_checker::Status::Cheat(_)) => Status::Cheat,
        _ => Status::Incorrect,
    };
    Ok(CreateCheckResponse { status })
}

/// Runs the checker without creating a submission.
#[utoipa::path(
    post,
    path = "/",
    tag = "challenge",
    params(("challenge_id" = i64, Path, description = "Challenge id")),
    request_body = CreateCheckRequest,
    responses(
        (status = 200, description = "Checker result", body = CreateCheckResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn create_check(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<CreateCheckRequest>,
) -> Result<Json<CreateCheckResponse>, WebError> {
    crate::middleware::auth::require_admin(&principal)?;
    let operator = principal
        .operator
        .ok_or(WebError::Unauthorized(serde_json::json!("unauthorized")))?;
    Ok(Json(run_check(&s, operator.id, challenge_id, &body.content).await?))
}

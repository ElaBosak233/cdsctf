//! HTTP handlers for `checker` within the `challenge_id` API segment.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_checker::traits::CheckerError;
use cds_db::{
    Challenge,
    sea_orm::{NotSet, Set, Unchanged},
};
use cds_engine::traits::{DiagnosticMarker, EngineError};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_checker).with_state(state.clone()))
        .routes(routes!(lint_checker).with_state(state.clone()))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateCheckerRequest {
    pub checker: Option<String>,
}

/// Updates checker.
#[utoipa::path(
    put,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = UpdateCheckerRequest,
    responses(
        (status = 200, description = "Checker updated", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_checker"))]
pub async fn update_checker(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateCheckerRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let _ = cds_db::challenge::update::<Challenge>(
        &s.db.conn,
        cds_db::challenge::ActiveModel {
            id: Unchanged(challenge_id),
            checker: body.checker.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct LintCheckerRequest {
    pub checker: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CheckerLintResponse {
    pub markers: Vec<DiagnosticMarker>,
}

/// Runs static analysis on a challenge checker script via API.
#[utoipa::path(
    post,
    path = "/lint",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = LintCheckerRequest,
    responses(
        (status = 200, description = "Lint markers (empty if clean)", body = CheckerLintResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "lint_checker"))]
pub async fn lint_checker(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<LintCheckerRequest>,
) -> Result<Json<CheckerLintResponse>, WebError> {
    let mut challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    challenge.checker = body.checker;

    let lint = s.checker.lint(&challenge).await;
    let diagnostics = if let Err(lint) = lint {
        match lint {
            CheckerError::EngineError(EngineError::DiagnosticsError(diagnostics)) => {
                Some(diagnostics)
            }
            _ => {
                error!("{:?}", lint);
                None
            }
        }
    } else {
        None
    };

    Ok(Json(CheckerLintResponse {
        markers: diagnostics.unwrap_or_default(),
    }))
}

//! HTTP handlers for `checker` within the `challenge_id` API segment.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_checker::traits::CheckerError;
use cds_db::{
    ChallengeDetail,
    sea_orm::{NotSet, Set, Unchanged},
};
use cds_engine::traits::{DiagnosticMarker, EngineError};
use garde::Validate;
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path, VJson},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_checker).with_state(state.clone()))
        .routes(routes!(lint_checker).with_state(state.clone()))
}

fn require_admin(principal: &AuthPrincipal) -> Result<(), WebError> {
    let operator = principal
        .operator
        .as_ref()
        .ok_or_else(|| WebError::Unauthorized(serde_json::json!("unauthorized")))?;

    if operator.group < cds_db::user::Group::Admin {
        return Err(WebError::Forbidden(serde_json::json!("forbidden")));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use cds_db::{UserAccountView, user::Group};

    use super::require_admin;
    use crate::traits::AuthPrincipal;

    fn principal(group: Group) -> AuthPrincipal {
        AuthPrincipal {
            operator: Some(UserAccountView {
                id: 1,
                name: "operator".to_owned(),
                username: "operator".to_owned(),
                verified: Some(true),
                group,
                description: None,
                hashed_password: String::new(),
                avatar_hash: None,
                created_at: time::OffsetDateTime::UNIX_EPOCH,
                updated_at: time::OffsetDateTime::UNIX_EPOCH,
            }),
            client_ip: String::new(),
        }
    }

    #[test]
    fn checker_operations_require_global_admin() {
        assert!(require_admin(&principal(Group::Admin)).is_ok());
        assert!(matches!(
            require_admin(&principal(Group::User)),
            Err(crate::traits::WebError::Forbidden(_))
        ));
        assert!(matches!(
            require_admin(&AuthPrincipal::default()),
            Err(crate::traits::WebError::Unauthorized(_))
        ));
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct UpdateCheckerRequest {
    pub checker: Option<String>,
}

/// Updates checker.
#[utoipa::path(
    put,
    path = "/",
    tag = "challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = UpdateCheckerRequest,
    responses(
        (status = 200, description = "Checker updated", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_checker"))]
pub async fn update_checker(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateCheckerRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    require_admin(&principal)?;
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let _ = cds_db::challenge::update::<ChallengeDetail>(
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
#[garde(allow_unvalidated)]
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
    tag = "challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = LintCheckerRequest,
    responses(
        (status = 200, description = "Lint markers (empty if clean)", body = CheckerLintResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "lint_checker"))]
pub async fn lint_checker(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<LintCheckerRequest>,
) -> Result<Json<CheckerLintResponse>, WebError> {
    require_admin(&principal)?;
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

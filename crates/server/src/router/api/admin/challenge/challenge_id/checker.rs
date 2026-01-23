use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_checker::traits::CheckerError;
use cds_db::{
    Challenge, DB,
    sea_orm::{NotSet, Set, Unchanged},
};
use cds_engine::traits::{DiagnosticMarker, EngineError};
use serde::{Deserialize, Serialize};
use tracing::error;
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::put(update_checker))
        .route("/lint", axum::routing::post(lint_checker))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCheckerRequest {
    pub checker: Option<String>,
}

pub async fn update_checker(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateCheckerRequest>,
) -> Result<WebResponse<()>, WebError> {
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

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LintCheckerRequest {
    pub checker: Option<String>,
}

pub async fn lint_checker(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
    VJson(body): VJson<LintCheckerRequest>,
) -> Result<WebResponse<Vec<DiagnosticMarker>>, WebError> {
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

    Ok(WebResponse {
        data: diagnostics,
        ..Default::default()
    })
}

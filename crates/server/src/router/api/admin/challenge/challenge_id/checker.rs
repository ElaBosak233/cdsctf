use axum::{Router, http::StatusCode};
use cds_checker::traits::{CheckerError, DiagnosticMarker};
use cds_db::{
    Challenge,
    sea_orm::{NotSet, Set, Unchanged},
};
use serde::{Deserialize, Serialize};
use tracing::error;
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_checker))
        .route("/lint", axum::routing::post(lint_checker))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCheckerRequest {
    pub checker: Option<String>,
}

pub async fn update_checker(
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateCheckerRequest>,
) -> Result<WebResponse<()>, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id).await?;

    let _ = cds_db::challenge::update::<Challenge>(cds_db::challenge::ActiveModel {
        id: Unchanged(challenge_id),
        checker: body.checker.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    })
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
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<LintCheckerRequest>,
) -> Result<WebResponse<Vec<DiagnosticMarker>>, WebError> {
    let mut challenge = crate::util::loader::prepare_challenge(challenge_id).await?;

    challenge.checker = body.checker;

    let lint = cds_checker::lint(&challenge).await;
    let diagnostics = if let Err(lint) = lint {
        match lint {
            CheckerError::DiagnosticsError(diagnostics) => Some(diagnostics),
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

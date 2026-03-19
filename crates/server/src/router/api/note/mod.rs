use std::sync::Arc;

use axum::{Router, extract::State, routing::get};
use cds_db::note::{FindNotesOptions, Note};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Query},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_note))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetNoteRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub size: Option<u64>,
    pub page: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_note(
    State(s): State<Arc<AppState>>,

    Extension(ap): Extension<AuthPrincipal>,
    Query(params): Query<GetNoteRequest>,
) -> Result<WebResponse<Vec<Note>>, WebError> {
    let _ = ap.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let (notes, total) = cds_db::note::find(
        &s.db.conn,
        FindNotesOptions {
            id: params.id,
            user_id: params.user_id,
            challenge_id: params.challenge_id,
            public: Some(true),
            size: params.size,
            page: params.page,
            sorts: params.sorts,
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse {
        data: Some(notes),
        total: Some(total),
        ..Default::default()
    })
}

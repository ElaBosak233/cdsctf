use std::sync::Arc;

use axum::{Router, extract::State, routing::put};
use cds_db::{
    Challenge,
    sea_orm::{Set, Unchanged},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", put(update_writeup))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWriteupRequest {
    pub writeup: String,
}

pub async fn update_writeup(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateWriteupRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
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

    Ok(WebResponse {
        data: challenge,
        ..Default::default()
    })
}

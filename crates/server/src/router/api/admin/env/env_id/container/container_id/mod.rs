use axum::{Router, extract::WebSocketUpgrade, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Path, Query},
    traits::WebError,
};

pub fn router() -> Router {
    Router::new().route("/shell", axum::routing::get(get_shell))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShellRequest {
    pub command: String,
}

pub async fn get_shell(
    Path((pod_id, container_id)): Path<(String, String)>,
    Query(params): Query<GetShellRequest>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    Ok(ws.on_upgrade(move |socket| async move {
        let _ = cds_cluster::exec(&pod_id, &container_id, params.command, socket).await;
    }))
}

use std::sync::Arc;

use axum::{
    Router,
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Path, Query},
    traits::{AppState, WebError},
};

#[allow(dead_code)]
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/shell", axum::routing::get(get_shell))
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShellRequest {
    pub command: String,
}

#[allow(dead_code)]
pub async fn get_shell(
    State(s): State<Arc<AppState>>,

    Path((instance_id, container_id)): Path<(String, String)>,
    Query(params): Query<GetShellRequest>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    Ok(ws.on_upgrade(move |socket| async move {
        let _ = s
            .cluster
            .exec(&instance_id, &container_id, params.command, socket)
            .await;
    }))
}

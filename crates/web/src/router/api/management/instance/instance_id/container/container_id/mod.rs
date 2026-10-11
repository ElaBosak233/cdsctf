//! HTTP routing for `container_id` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Path, Query},
    traits::{AppState, WebError},
};

#[allow(dead_code)]
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShellRequest {
    pub command: String,
}

/// Returns shell.

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

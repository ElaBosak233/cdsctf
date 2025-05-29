use axum::{Router, extract::WebSocketUpgrade, response::IntoResponse};
use cds_db::entity::user::Group;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Path, Query},
    traits::{AuthPrincipal, WebError},
};

pub fn router() -> Router {
    Router::new().route("/shell", axum::routing::get(get_shell))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShellRequest {
    pub command: String,
}

pub async fn get_shell(
    Extension(ext): Extension<AuthPrincipal>,
    Path((pod_id, container_id)): Path<(String, String)>,
    Query(params): Query<GetShellRequest>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    Ok(ws.on_upgrade(move |socket| async move {
        let _ = cds_cluster::exec(&pod_id, &container_id, params.command, socket).await;
    }))
}

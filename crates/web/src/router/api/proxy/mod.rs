use axum::{
    Router,
    extract::{Path, Query, WebSocketUpgrade},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use tracing::debug;

use crate::traits::WebError;

pub fn router() -> Router {
    Router::new().route("/:token", axum::routing::get(link))
}

#[derive(Deserialize)]
pub struct LinkRequest {
    pub port: u32,
}

pub async fn link(
    Path(token): Path<String>, Query(query): Query<LinkRequest>, ws: Option<WebSocketUpgrade>,
) -> Result<impl IntoResponse, WebError> {
    if ws.is_none() {
        return Err(WebError::BadRequest(json!("")));
    }

    let ws = ws.unwrap();
    let port = query.port;

    Ok(ws.on_upgrade(move |socket| async move {
        let result = cds_cluster::wsrx(token, port as u16, socket).await;
        if let Err(e) = result {
            debug!("Failed to link pods: {:?}", e);
        }
    }))
}

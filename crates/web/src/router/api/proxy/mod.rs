use axum::{Router, extract::WebSocketUpgrade, response::IntoResponse};
use serde::Deserialize;
use tracing::debug;
use uuid::Uuid;
use crate::{
    extract::{Path, Query},
    traits::WebError,
};

pub fn router() -> Router {
    Router::new().route("/{token}", axum::routing::get(link))
}

#[derive(Deserialize)]
pub struct LinkRequest {
    pub port: u32,
}

#[axum::debug_handler]
pub async fn link(
    Path(token): Path<Uuid>, Query(query): Query<LinkRequest>, ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, WebError> {
    let port = query.port;

    Ok(ws.on_upgrade(move |socket| async move {
        let result = cds_cluster::wsrx(token, port as u16, socket).await;
        if let Err(e) = result {
            debug!("Failed to link pods: {:?}", e);
        }
    }))
}

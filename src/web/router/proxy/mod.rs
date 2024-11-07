use axum::{
    extract::{Path, Query, WebSocketUpgrade},
    response::IntoResponse,
    Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing::debug;
use crate::{config, database::get_db, web::traits::WebError};

pub fn router() -> Router {
    return Router::new().route("/:token", axum::routing::get(link));
}

#[derive(Deserialize)]
pub struct LinkRequest {
    pub port: u32,
}

pub async fn link(
    Path(token): Path<String>, Query(query): Query<LinkRequest>, ws: Option<WebSocketUpgrade>,
) -> Result<impl IntoResponse, WebError> {
    if ws.is_none() {
        return Err(WebError::BadRequest(String::from("")));
    }

    let ws = ws.unwrap();
    let port = query.port;

    Ok(ws.on_upgrade(move |socket| async move {
        let result = crate::cluster::wsrx(token, port as u16, socket).await;
        if let Err(e) = result {
            debug!("Failed to link pods: {:?}", e);
        }
    }))
}

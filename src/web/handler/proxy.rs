use axum::{
    extract::{Path, Query, WebSocketUpgrade},
    response::IntoResponse,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config,
    database::get_db,
    web::{model::proxy::*, traits::WebError},
};

pub async fn link(
    Path(token): Path<String>, Query(query): Query<LinkRequest>, ws: Option<WebSocketUpgrade>,
) -> Result<impl IntoResponse, WebError> {
    if ws.is_none() {
        return Err(WebError::BadRequest(String::from("")));
    }

    let ws = ws.unwrap();

    let pod = crate::model::pod::Entity::find()
        .filter(crate::model::pod::Column::Name.eq(token))
        .one(&get_db())
        .await
        .unwrap();

    if pod.is_none() {
        return Err(WebError::NotFound(String::from("")));
    }

    let pod = pod.unwrap();

    let target_nat = pod.nats.iter().find(|p| p.src == query.port.to_string());

    if target_nat.is_none() {
        return Err(WebError::NotFound(String::from("")));
    }

    let target_nat = target_nat.unwrap();
    let target_port = target_nat.dst.clone().unwrap();
    let target_url = format!("{}:{}", config::get_config().container.entry, target_port);

    return Ok(ws.on_upgrade(move |socket| async move {
        let tcp = tokio::net::TcpStream::connect(target_url).await.unwrap();
        let _ = wsrx::proxy(socket.into(), tcp).await;
    }));
}

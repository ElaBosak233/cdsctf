use axum::{
    extract::{Path, Query, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;

use crate::{config, database::get_db};

#[derive(Deserialize)]
pub struct LinkQuery {
    port: u32,
}

pub async fn link(
    Path(token): Path<String>, Query(query): Query<LinkQuery>, ws: Option<WebSocketUpgrade>,
) -> Response {
    if ws.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
            })),
        )
            .into_response();
    }

    let ws = ws.unwrap();

    let pod = crate::model::pod::Entity::find()
        .filter(crate::model::pod::Column::Name.eq(token))
        .one(&get_db())
        .await
        .unwrap();

    if pod.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "code": StatusCode::NOT_FOUND.as_u16(),
            })),
        )
            .into_response();
    }

    let pod = pod.unwrap();

    let target_nat = pod.nats.iter().find(|p| p.src == query.port.to_string());

    if target_nat.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "code": StatusCode::NOT_FOUND.as_u16(),
            })),
        )
            .into_response();
    }

    let target_nat = target_nat.unwrap();
    let target_port = target_nat.dst.clone().unwrap();
    let target_url = format!("{}:{}", config::get_config().container.entry, target_port);

    return ws.on_upgrade(move |socket| async move {
        let tcp = tokio::net::TcpStream::connect(target_url).await.unwrap();
        let _ = wsrx::proxy(socket.into(), tcp).await;
    });
}

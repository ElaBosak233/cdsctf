use axum::{
    Router,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::extract::Path;

pub fn router() -> Router {
    Router::new().route("/{*path}", axum::routing::get(get_file))
}

pub async fn get_file(Path(path): Path<String>) -> impl IntoResponse {
    let filename = path.split("/").last().unwrap_or("attachment");
    let dir = path.rfind('/').map(|i| path[..i].to_string()).unwrap_or_default();
    match cds_media::get(dir, filename.to_string()).await {
        Ok(buffer) => Response::builder()
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .header(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            )
            .body(buffer.into())
            .unwrap(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

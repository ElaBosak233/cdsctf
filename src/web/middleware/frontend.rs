use std::{fs, path::PathBuf};

use axum::{
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

pub async fn serve(req: Request, next: Next) -> Result<axum::response::Response, StatusCode> {
    let path: String = req.uri().path().to_string();

    if path.starts_with("/api") {
        return Ok(next.run(req).await);
    }

    let filepath = PathBuf::from("dist").join(path.strip_prefix("/").unwrap_or_default());

    async fn index() -> Result<axum::response::Response, StatusCode> {
        if let Ok(index_content) = fs::read_to_string(PathBuf::from("dist").join("index.html")) {
            return Ok(Response::builder().status(StatusCode::OK).body(index_content).unwrap().into_response());
        } else {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("404 Not Found".to_string())
                .unwrap()
                .into_response());
        }
    }

    if filepath == PathBuf::from("dist").join("index.html") {
        return index().await;
    }

    if let Ok(content) = fs::read_to_string(&filepath) {
        return Ok(Response::builder().status(StatusCode::OK).body(content).unwrap().into_response());
    } else {
        return index().await;
    }
}

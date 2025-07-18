use axum::{
    Router,
    body::Body,
    http::Response,
    response::{IntoResponse, Redirect},
};

use crate::traits::WebError;

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_logo))
}

pub async fn get_logo() -> Result<impl IntoResponse, WebError> {
    let path = "configs/logo".to_owned();
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Ok(Redirect::to("/logo.svg").into_response()),
    }
}

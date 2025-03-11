use axum::{
    Router,
    body::Body,
    extract::Multipart,
    http::Response,
    response::{IntoResponse, Redirect},
};
use cds_db::entity::user::Group;

use crate::{
    extract::Extension,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_logo))
        .route("/", axum::routing::post(save_logo))
        .route("/", axum::routing::delete(delete_logo))
}

pub async fn get_logo() -> Result<impl IntoResponse, WebError> {
    let path = "logo".to_owned();
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Ok(Redirect::to("/logo.svg").into_response()),
    }
}

pub async fn save_logo(
    Extension(ext): Extension<Ext>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden("".into()));
    }

    util::media::save_img("logo".to_owned(), multipart).await
}

pub async fn delete_logo(Extension(ext): Extension<Ext>) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden("".into()));
    }

    util::media::delete_img("logo".to_owned()).await
}

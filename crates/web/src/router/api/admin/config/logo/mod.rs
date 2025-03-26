use axum::{Router, extract::Multipart};
use cds_db::entity::user::Group;

use crate::{
    extract::Extension,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(save_logo))
        .route("/", axum::routing::delete(delete_logo))
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

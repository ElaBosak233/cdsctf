use axum::Router;

use crate::{
    extract::{Extension, Path},
    model::user::User,
    traits::{AuthPrincipal, WebError, WebResponse},
};

mod avatar;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .nest("/avatar", avatar::router())
}

pub async fn get_user(
    Extension(ext): Extension<AuthPrincipal>,
    Path(user_id): Path<i64>,
) -> Result<WebResponse<User>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let user = crate::util::loader::prepare_user(user_id).await?;

    Ok(WebResponse {
        data: Some(user),
        ..Default::default()
    })
}

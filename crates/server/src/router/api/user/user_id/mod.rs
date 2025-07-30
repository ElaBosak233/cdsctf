use axum::Router;
use cds_db::User;

use crate::{
    extract::{Extension, Path},
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

    let user = cds_db::user::find_by_id::<User>(user_id).await?;

    Ok(WebResponse {
        data: user,
        ..Default::default()
    })
}

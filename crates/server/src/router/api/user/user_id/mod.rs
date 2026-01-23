use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::{DB, User};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

mod avatar;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .nest("/avatar", avatar::router())
}

pub async fn get_user(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(user_id): Path<i64>,
) -> Result<WebResponse<User>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let user = cds_db::user::find_by_id(&s.db.conn, user_id).await?;

    Ok(WebResponse {
        data: user,
        ..Default::default()
    })
}

use std::sync::Arc;

use axum::{body::Body, extract::State, http::Response, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "Avatar image bytes"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_user_avatar(
    State(s): State<Arc<AppState>>,

    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}", user_id);

    let buffer = s.media.get(path, "avatar".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}

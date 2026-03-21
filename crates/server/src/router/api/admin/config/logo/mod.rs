use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    traits::{AppState, EmptySuccess, WebError},
    util::media::handle_multipart,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::post(save_logo))
        .route("/", axum::routing::delete(delete_logo))
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(save_logo).with_state(state.clone()))
        .routes(routes!(delete_logo).with_state(state.clone()))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-config",
    responses(
        (status = 200, description = "Logo saved", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn save_logo(
    State(s): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<EmptySuccess>, WebError> {
    let data = handle_multipart(multipart, mime::IMAGE).await?;

    s.media
        .save("configs".to_owned(), "logo".to_owned(), data)
        .await?;

    Ok(Json(EmptySuccess::default()))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-config",
    responses(
        (status = 200, description = "Logo removed", body = EmptySuccess),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn delete_logo(State(s): State<Arc<AppState>>) -> Result<Json<EmptySuccess>, WebError> {
    s.media
        .delete("configs".to_owned(), "logo".to_owned())
        .await?;

    Ok(Json(EmptySuccess::default()))
}

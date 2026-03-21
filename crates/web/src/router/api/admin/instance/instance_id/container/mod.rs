mod container_id;

use std::sync::Arc;

use axum::{Json, Router};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::traits::{AppState, EmptyJson, WebError};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_container).with_state(state.clone()))
        .nest(
            "/{container_id}",
            OpenApiRouter::from(Router::new().with_state(state.clone())),
        )
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-instance",
    responses(
        (status = 200, description = "OK", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_container() -> Result<Json<EmptyJson>, WebError> {
    Ok(Json(EmptyJson::default()))
}

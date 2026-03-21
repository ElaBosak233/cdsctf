use std::sync::Arc;

use axum::{Router, body::Body, extract::State, http::Response, response::IntoResponse};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game_poster).with_state(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Poster bytes"),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_game_poster(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}", game_id);

    let buffer = s.media.get(path, "poster".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}

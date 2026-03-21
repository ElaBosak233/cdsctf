use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_team_write_up))
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_team_write_up).with_state(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Write-up file"),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_team_write_up(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    util::media::get_write_up(s.media.clone(), game_id, team_id).await
}

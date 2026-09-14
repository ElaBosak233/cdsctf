//! HTTP routing for users who belong to a game.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::user::{FindUserOptions, Group, UserAccountView};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Path, Query},
    traits::{AppState, WebError},
};

/// Builds the game-scoped user search router.
pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game_users).with_state(state))
}

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameUsersRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameUsersListResponse {
    pub users: Vec<UserAccountView>,
    pub total: u64,
}

/// Returns users that belong to at least one team in the requested game.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetGameUsersRequest,
    ),
    responses(
        (status = 200, description = "Game users", body = GameUsersListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn get_game_users(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameUsersRequest>,
) -> Result<Json<GameUsersListResponse>, WebError> {
    let _ = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (users, total) = cds_db::user::find_by_game_id::<UserAccountView>(
        &s.db.conn,
        game_id,
        FindUserOptions {
            id: params.id,
            name: params.name,
            group: params.group,
            sorts: params.sorts,
            page: Some(page),
            size: Some(size),
        },
    )
    .await?;

    Ok(Json(GameUsersListResponse { users, total }))
}

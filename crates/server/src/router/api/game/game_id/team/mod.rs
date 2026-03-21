mod team_id;
pub mod us;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    TeamUser,
    sea_orm::ActiveValue::Set,
    team::{State as TState, Team},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", axum::routing::post(team_register))
        .nest("/us", us::router())
        .nest("/{team_id}", team_id::router())
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(team_register).with_state(state.clone()))
        .nest("/us", us::openapi_router(state.clone()))
        .nest("/{team_id}", team_id::openapi_router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TeamRegisterRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = TeamRegisterRequest,
    responses(
        (status = 200, description = "Team created", body = TeamResponse),
        (status = 400, description = "Bad request", body = crate::traits::ApiJsonError),
        (status = 401, description = "Unauthorized", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn team_register(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<TeamRegisterRequest>,
) -> Result<Json<TeamResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    if cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let team = cds_db::team::create::<Team>(
        &s.db.conn,
        cds_db::team::ActiveModel {
            name: Set(body.name),
            email: Set(body.email),
            slogan: Set(body.slogan),
            game_id: Set(game.id),
            state: Set(TState::Preparing),
            ..Default::default()
        },
    )
    .await?;

    let _ = cds_db::team_user::create::<TeamUser>(
        &s.db.conn,
        cds_db::team_user::ActiveModel {
            team_id: Set(team.id),
            user_id: Set(operator.id),
        },
    )
    .await?;

    let team = cds_db::team::find_by_id(&s.db.conn, team.id, team.game_id)
        .await?
        .ok_or(WebError::NotFound(json!("")))?;

    Ok(Json(TeamResponse { team }))
}

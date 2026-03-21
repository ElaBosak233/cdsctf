mod token;
mod user;
mod writeup;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Team,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    team::State as TState,
};
use cds_worker::calculator::Payload;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Path},
    router::api::game::game_id::team::TeamResponse,
    traits::{AppState, EmptyJson, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_team).with_state(state.clone()))
        .routes(routes!(delete_team).with_state(state.clone()))
        .nest("/users", user::router(state.clone()))
        .nest("/token", token::router(state.clone()))
        .nest("/writeup", writeup::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub state: Option<TState>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    put,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    request_body = UpdateTeamRequest,
    responses(
        (status = 200, description = "Updated team", body = TeamResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn update_team(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    ReqJson(body): ReqJson<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let new_team = cds_db::team::update::<Team>(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            game_id: Unchanged(team.game_id),
            name: body.name.map_or(NotSet, Set),
            state: body.state.map_or(NotSet, Set),
            slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
            email: body.email.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    if team.state != new_team.state {
        s.queue
            .publish(
                "calculator",
                Payload {
                    game_id: Some(game_id),
                },
            )
            .await?;
    }

    Ok(Json(TeamResponse { team: new_team }))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn delete_team(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<EmptyJson>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete_by_team_id(&s.db.conn, team.id).await?;

    cds_db::team::delete(&s.db.conn, team.id).await?;

    Ok(Json(EmptyJson::default()))
}

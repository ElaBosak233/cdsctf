use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{TeamUser, team::State as TState, team_user::FindTeamUserOptions};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(leave_team).with_state(state.clone()))
}

#[utoipa::path(
    delete,
    path = "/leave",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Left team", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn leave_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    let (_, count) = cds_db::team_user::find::<TeamUser>(
        &s.db.conn,
        FindTeamUserOptions {
            team_id: Some(team.id),
            user_id: Some(operator.id),
        },
    )
    .await?;

    if count <= 1 {
        return Err(WebError::BadRequest(json!("team_has_no_other_member")));
    }

    cds_db::team_user::delete(&s.db.conn, team.id, operator.id).await?;

    Ok(Json(EmptyJson::default()))
}

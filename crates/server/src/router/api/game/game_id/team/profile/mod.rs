mod avatar;
mod token;
mod user;
mod writeup;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::{
    TeamUser,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    team::{State as TState, Team},
    team_user::FindTeamUserOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_team))
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .route("/ready", axum::routing::post(set_team_ready))
        .nest("/avatar", avatar::router())
        .nest("/users", user::router())
        .nest("/token", token::router())
        .nest("/writeup", writeup::router())
}

pub async fn get_team(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    Ok(WebResponse {
        data: Some(team),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Update a team with given path and data.
pub async fn update_team(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    let team = cds_db::team::update(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            game_id: Unchanged(team.game_id),
            name: body.name.map_or(NotSet, Set),
            slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
            email: body.email.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

pub async fn delete_team(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete_by_team_id(&s.db.conn, team.id).await?;

    cds_db::team::delete(&s.db.conn, team.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

/// Set a team's state to Pending.
///
/// # Prerequisite
/// - Operator is admin or one of the members of current team.
pub async fn set_team_ready(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    // Review the number of members
    let (_, team_users) = cds_db::team_user::find::<TeamUser>(
        &s.db.conn,
        FindTeamUserOptions {
            team_id: Some(team.id),
            ..Default::default()
        },
    )
    .await?;

    if team_users < game.member_limit_min as u64 || team_users > game.member_limit_max as u64 {
        return Err(WebError::BadRequest(json!("member_limit_not_satisfied")));
    }

    let team = cds_db::team::update(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            game_id: Unchanged(team.game_id),
            state: Set(if game.is_public {
                TState::Passed
            } else {
                TState::Pending
            }),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

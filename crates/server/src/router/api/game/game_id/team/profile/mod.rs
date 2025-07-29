mod avatar;
mod token;
mod user;

use axum::{Router, http::StatusCode};
use cds_db::{
    TeamUser,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    team::{State, Team},
    team_user::FindTeamUserOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team))
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .route("/ready", axum::routing::post(set_team_ready))
        .nest("/avatar", avatar::router())
        .nest("/users", user::router())
        .nest("/token", token::router())
}

pub async fn get_team(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

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
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    let team = cds_db::team::update(cds_db::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        name: body.name.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

pub async fn delete_team(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete_by_team_id(team.id).await?;

    cds_db::team::delete(team.id).await?;

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
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(game_id).await?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    // Review the number of members
    let (_, team_users) = cds_db::team_user::find::<TeamUser>(FindTeamUserOptions {
        team_id: Some(team.id),
        ..Default::default()
    })
    .await?;

    if team_users < game.member_limit_min as u64 || team_users > game.member_limit_max as u64 {
        return Err(WebError::BadRequest(json!("member_limit_not_satisfied")));
    }

    let team = cds_db::team::update::<Team>(cds_db::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        state: Set(if game.is_public {
            State::Passed
        } else {
            State::Pending
        }),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

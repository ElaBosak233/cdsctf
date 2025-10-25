mod avatar;
mod token;
mod user;
mod writeup;

use axum::{Router, http::StatusCode};
use cds_db::{
    Team,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    team::State,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Json, Path},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .nest("/avatar", avatar::router())
        .nest("/users", user::router())
        .nest("/token", token::router())
        .nest("/writeup", writeup::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub state: Option<State>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Update a team with given path and data.
///
/// # Prerequisite
/// - Operator is admin or one of current team.
pub async fn update_team(
    Path((game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    let new_team = cds_db::team::update::<Team>(cds_db::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        name: body.name.map_or(NotSet, Set),
        state: body.state.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    })
    .await?;

    if team.state != new_team.state {
        cds_queue::publish(
            "calculator",
            crate::worker::game_calculator::Payload {
                game_id: Some(game_id),
            },
        )
        .await?;
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(new_team),
        ..Default::default()
    })
}

pub async fn delete_team(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

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

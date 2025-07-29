use axum::{Router, http::StatusCode};
use cds_db::{TeamUser, sea_orm::ActiveValue::Set, team::State};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Json, Path},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_team_user))
        .route("/{user_id}", axum::routing::delete(delete_team_user))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamUserRequest {
    pub user_id: i64,
}

/// Add a user into a team by given data.
///
/// Only admins can use this function.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team_user(
    Path((game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<CreateTeamUserRequest>,
) -> Result<WebResponse<()>, WebError> {
    let user = crate::util::loader::prepare_user(body.user_id).await?;
    let game = crate::util::loader::prepare_game(game_id).await?;
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    if cds_db::util::is_user_in_game(user.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let _ = cds_db::team_user::create::<TeamUser>(cds_db::team_user::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(team.id),
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

/// Kick a user from a team by `id` and `user_id`.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_team_user(
    Path((game_id, team_id, user_id)): Path<(i64, i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete(team_id, user_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

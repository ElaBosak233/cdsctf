use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{team::State, user::Group},
    get_db,
    sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{Ext, WebError, WebResponse},
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
    Extension(ext): Extension<Ext>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<CreateTeamUserRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let user = crate::util::loader::prepare_user(operator.id).await?;
    let game = crate::util::loader::prepare_game(game_id).await?;
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    if cds_db::util::is_user_in_game(user.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let _ = cds_db::entity::team_user::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(team.id),
    }
    .insert(get_db())
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
    Extension(ext): Extension<Ext>,
    Path((game_id, team_id, user_id)): Path<(i64, i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team_id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

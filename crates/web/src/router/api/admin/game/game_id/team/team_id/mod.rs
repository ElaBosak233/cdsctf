mod avatar;
mod token;
mod user;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{team::State, user::Group},
    get_db,
    sea_orm::{
        ActiveModelTrait,
        ActiveValue::{Set, Unchanged},
        ColumnTrait, EntityTrait, NotSet, QueryFilter,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    model::team::Team,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .nest("/avatar", avatar::router())
        .nest("/users", user::router())
        .nest("/token", token::router())
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
    Extension(ext): Extension<Ext>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        name: body.name.map_or(NotSet, Set),
        state: body.state.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    let team = crate::util::loader::prepare_team(game_id, team.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

pub async fn delete_team(
    Extension(ext): Extension<Ext>,
    Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin
        && !cds_db::util::is_user_in_team(operator.id, team_id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .exec(get_db())
        .await?;

    let _ = cds_db::entity::team::Entity::delete_many()
        .filter(cds_db::entity::team::Column::GameId.eq(team.game_id))
        .filter(cds_db::entity::team::Column::Id.eq(team.id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{team::State, user::Group},
    get_db,
    transfer::Team,
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, Condition, EntityTrait, NotSet, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{Ext, WebError, WebResponse},
};

mod avatar;
mod token;
mod user;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .route("/state", axum::routing::put(update_team_state))
        .nest("/avatar", avatar::router())
        .nest("/users", user::router())
        .nest("/token", token::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Update a team with given path and data.
///
/// # Prerequisite
/// - Operator is admin or one of current team.
pub async fn update_team(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = cds_db::entity::team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::team::Column::GameId.eq(game_id))
                .add(cds_db::entity::team::Column::Id.eq(team_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        name: body.name.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

pub async fn delete_team(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = cds_db::entity::team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::team::Column::GameId.eq(game_id))
                .add(cds_db::entity::team::Column::Id.eq(team_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamStateRequest {
    pub state: State,
}

/// Update a team's state with given path and data.
///
/// This function is only used to switch whether
/// the team is allowed to access the game or not.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_team_state(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateTeamStateRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin
        && !cds_db::util::is_user_in_team(operator.id, team_id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = cds_db::entity::team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::team::Column::GameId.eq(game_id))
                .add(cds_db::entity::team::Column::Id.eq(team_id)),
        )
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        state: Set(body.state),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

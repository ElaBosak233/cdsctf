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
        ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter,
    },
    transfer::Team,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{Ext, WebError, WebResponse},
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
    Extension(ext): Extension<Ext>,
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
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        name: body.name.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
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
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

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

/// Set a team's state to Pending.
///
/// # Prerequisite
/// - Operator is admin or one of the members of current team.
pub async fn set_team_ready(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(game_id).await?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    // Review the number of members
    let team_users = cds_db::entity::team_user::Entity::find()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .count(get_db())
        .await?;

    if team_users < game.member_limit_min as u64 || team_users > game.member_limit_max as u64 {
        return Err(WebError::BadRequest(json!("member_limit_not_satisfied")));
    }

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        game_id: Unchanged(team.game_id),
        state: Set(if game.is_public {
            State::Passed
        } else {
            State::Pending
        }),
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

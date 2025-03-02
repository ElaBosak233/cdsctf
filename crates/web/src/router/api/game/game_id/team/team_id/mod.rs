use axum::{http::StatusCode, Router};
use cds_db::{entity::user::Group, get_db, transfer::GameTeam};
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

pub mod avatar;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_game_team))
        .route("/", axum::routing::delete(delete_game_team))
        .nest("/avatar", avatar::router())
        // .route("/join")
        // .route("/leave")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGameTeamRequest {
    pub game_id: Option<i64>,
    pub game_team_id: Option<i64>,
    pub is_allowed: Option<bool>,
}

/// Update a game team with given path and data.
///
/// This function is only used to switch whether
/// the game_team is allowed to access the game or not.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_game_team(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateGameTeamRequest>,
) -> Result<WebResponse<GameTeam>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_team = cds_db::entity::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_team::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_team::Column::Id.eq(team_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_team_not_found")))?;

    let game_team = cds_db::entity::game_team::ActiveModel {
        id: Unchanged(game_team.id),
        game_id: Unchanged(game_team.game_id),
        is_allowed: body.is_allowed.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game_team = cds_db::transfer::GameTeam::from(game_team);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_team),
        ..Default::default()
    })
}

pub async fn delete_game_team(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_team = cds_db::entity::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_team::Column::GameId.eq(game_id))
                .add(cds_db::entity::game_team::Column::Id.eq(team_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_team_not_found")))?;

    let _ = cds_db::entity::game_team::Entity::delete_many()
        .filter(cds_db::entity::game_team::Column::GameId.eq(game_team.game_id))
        .filter(cds_db::entity::game_team::Column::Id.eq(game_team.id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

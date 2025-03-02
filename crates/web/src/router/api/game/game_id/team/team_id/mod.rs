use axum::{http::StatusCode, Router};
use cds_db::{entity::user::Group, get_db, transfer::Team};
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
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .nest("/avatar", avatar::router())
        // .route("/join")
        // .route("/leave")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGameTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_allowed: Option<bool>,
}

/// Update a game team with given path and data.
///
/// This function is only used to switch whether
/// the team is allowed to access the game or not.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_team(
    Extension(ext): Extension<Ext>, Path((game_id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateGameTeamRequest>,
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
        is_allowed: body.is_allowed.map_or(NotSet, Set),
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

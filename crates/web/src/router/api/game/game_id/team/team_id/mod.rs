mod avatar;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::team::State,
    get_db,
    sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    model::user::UserMini,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .nest("/avatar", avatar::router())
        .route("/members", axum::routing::get(get_team_members))
        .route("/join", axum::routing::post(join_team))
}

pub async fn get_team_members(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<Vec<UserMini>>, WebError> {
    let users = cds_db::entity::user::Entity::find()
        .inner_join(cds_db::entity::team::Entity)
        .filter(cds_db::entity::team::Column::Id.eq(team_id))
        .filter(cds_db::entity::team::Column::GameId.eq(game_id))
        .into_model::<UserMini>()
        .all(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(users),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinTeamRequest {
    pub team_id: i64,
    pub token: String,
}

pub async fn join_team(
    Extension(ext): Extension<Ext>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<JoinTeamRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(game_id).await?;
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    if cds_db::util::is_user_in_game(operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let criteria = cds_cache::get::<String>(format!("team:{}:invite", body.team_id))
        .await?
        .ok_or(WebError::BadRequest(json!("no_invite_token")))?;

    if criteria != body.token {
        return Err(WebError::BadRequest(json!("invalid_invite_token")));
    }

    let _ = cds_db::entity::team_user::ActiveModel {
        team_id: Set(team.id),
        user_id: Set(operator.id),
    }
    .insert(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

mod profile;
mod team_id;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::team::State,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
        QuerySelect, RelationTrait,
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
        .route("/register", axum::routing::post(team_register))
        .nest("/profile", profile::router())
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamRegisterRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Add a team to a game with given path and data.
///
/// # Prerequisite
/// - No user in the team is already in the game.
pub async fn team_register(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Json(body): Json<TeamRegisterRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(game_id).await?;

    if cds_db::util::is_user_in_game(operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        slogan: Set(body.slogan),
        game_id: Set(game.id),
        state: Set(State::Preparing),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let _ = cds_db::entity::team_user::ActiveModel {
        team_id: Set(team.id),
        user_id: Set(operator.id),
    }
    .insert(get_db())
    .await?;

    let team = cds_db::entity::team::Entity::find_by_id(team.id)
        .into_model::<Team>()
        .one(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: team,
        ..Default::default()
    })
}

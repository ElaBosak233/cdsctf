use axum::{Router, http::StatusCode};
use cds_db::{entity::team::State, get_db, sea_orm::EntityTrait};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::challenge::Challenge,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_challenge))
}

pub async fn get_challenge(
    Extension(ext): Extension<Ext>,
    Path((game_id, challenge_id)): Path<(i64, uuid::Uuid)>,
) -> Result<WebResponse<Challenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(game_id).await?;

    let now = chrono::Utc::now().timestamp();
    let in_game = cds_db::util::is_user_in_game(operator.id, game.id, Some(State::Passed)).await?;

    if !in_game || !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = crate::util::loader::prepare_game_challenge(game_id, challenge_id).await?;

    let challenge =
        match cds_db::entity::challenge::Entity::find_by_id(game_challenge.challenge_id)
            .into_model::<Challenge>()
            .one(get_db())
            .await?
        {
            Some(challenge) => challenge,
            None => return Err(WebError::NotFound(json!("challenge_not_found"))),
        }
        .desensitize();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenge),
        ..Default::default()
    })
}

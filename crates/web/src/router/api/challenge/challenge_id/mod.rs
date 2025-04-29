mod attachment;

use axum::{Router, http::StatusCode};
use cds_db::{get_db, sea_orm::EntityTrait};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::challenge::Challenge,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge))
        .nest("/attachment", attachment::router())
}

// Here's a small issue. After the game starts, the players can bring out
// challenge ids for someone outside to do
pub async fn get_challenge(
    Extension(ext): Extension<Ext>,
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<Challenge>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = match cds_db::entity::challenge::Entity::find_by_id(challenge_id)
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

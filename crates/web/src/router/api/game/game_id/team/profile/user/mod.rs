use axum::{Router, http::StatusCode};
use cds_db::{
    entity::team::State,
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter},
};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new().route("/leave", axum::routing::delete(leave_team))
}

pub async fn leave_team(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    if team.state != State::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    let count = cds_db::entity::team_user::Entity::find()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .count(get_db())
        .await?;

    if count <= 1 {
        return Err(WebError::BadRequest(json!("team_has_no_other_member")));
    }

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::UserId.eq(operator.id))
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

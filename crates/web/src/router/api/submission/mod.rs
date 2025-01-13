pub mod checker;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path, Query, VJson},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    checker::init().await;

    Router::new()
        .route("/", axum::routing::get(get))
        .route("/{id}", axum::routing::get(get_by_id))
        .route("/", axum::routing::post(create))
        .route("/{id}", axum::routing::delete(delete))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Submission>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(json!("")));
    }

    let (mut submissions, total) = cds_db::transfer::submission::find(
        params.id,
        params.user_id,
        params.team_id,
        params.game_id,
        params.challenge_id,
        params.status,
        params.page,
        params.size,
    )
    .await?;

    let is_detailed = params.is_detailed.unwrap_or(false);
    for submission in submissions.iter_mut() {
        if !is_detailed {
            submission.desensitize();
        }
    }

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submissions),
        total: Some(total),
        ..WebResponse::default()
    })
}

pub async fn get_by_id(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<cds_db::transfer::Submission>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let submission = cds_db::entity::submission::Entity::find_by_id(id)
        .one(get_db())
        .await?;

    if submission.is_none() {
        return Err(WebError::NotFound(json!("")));
    }

    let submission = submission.unwrap();
    let mut submission = cds_db::transfer::Submission::from(submission);
    submission.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub flag: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreateRequest>,
) -> Result<WebResponse<cds_db::transfer::Submission>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    body.user_id = Some(operator.id);

    if let Some(challenge_id) = body.challenge_id.clone() {
        let challenge = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
            .one(get_db())
            .await?;

        if challenge.is_none() {
            return Err(WebError::BadRequest(json!("challenge_not_found")));
        }
    } else {
        return Err(WebError::BadRequest(json!("challenge_id_required")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let game = cds_db::entity::game::Entity::find_by_id(game_id)
            .one(get_db())
            .await?;

        if game.is_none() {
            return Err(WebError::BadRequest(json!("game_not_found")));
        }

        let team = cds_db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?;

        if team.is_none() {
            return Err(WebError::BadRequest(json!("team_not_found")));
        }

        let game_challenge = cds_db::entity::game_challenge::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
                    .add(
                        cds_db::entity::game_challenge::Column::ChallengeId
                            .eq(body.challenge_id.unwrap()),
                    ),
            )
            .one(get_db())
            .await?;

        if game_challenge.is_none() {
            return Err(WebError::BadRequest(json!("game_challenge_not_found")));
        }
    }

    let submission = cds_db::entity::submission::ActiveModel {
        flag: Set(body.flag),
        user_id: body.user_id.map_or(NotSet, Set),
        team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
        game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
        challenge_id: body.challenge_id.map_or(NotSet, Set),
        status: Set(Status::Pending),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let submission = cds_db::transfer::Submission::from(submission);

    cds_queue::publish("checker", submission.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..WebResponse::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::submission::Entity::delete_by_id(id)
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

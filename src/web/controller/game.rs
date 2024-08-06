use crate::database::get_db;
use crate::model::game::request::GetSubmissionRequest;
use crate::web::traits::Ext;
use crate::web::traits::WebError;
use axum::body::Body;
use axum::{
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde_json::json;

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<crate::model::game::request::FindRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && !params.is_enabled.unwrap_or(true) {
        return Err(WebError::Forbidden(String::new()));
    }

    let (challenges, total) = crate::model::game::find(
        params.id,
        params.title,
        params.is_enabled,
        params.page,
        params.size,
    )
    .await
    .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenges),
            "total": total,
        })),
    ));
}

pub async fn create(
    Json(body): Json<crate::model::game::request::CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let challenge = crate::model::game::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenge),
        })),
    ));
}

pub async fn update(
    Path(id): Path<i64>, Json(mut body): Json<crate::model::game::request::UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
    body.id = Some(id);

    let challenge = crate::model::game::ActiveModel::from(body)
        .update(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenge),
        })),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::game::Entity::delete_by_id(id)
        .exec(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn get_challenge(
    Query(params): Query<crate::model::game_challenge::request::FindRequest>,
) -> Result<impl IntoResponse, WebError> {
    let (challenges, _) =
        crate::model::game_challenge::find(params.game_id, params.challenge_id, params.is_enabled)
            .await
            .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenges),
        })),
    ));
}

pub async fn create_challenge(
    Json(body): Json<crate::model::game_challenge::request::CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let game_challenge = crate::model::game_challenge::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(game_challenge),
        })),
    ));
}

pub async fn update_challenge(
    Path((id, challenge_id)): Path<(i64, i64)>,
    Json(mut body): Json<crate::model::game_challenge::request::UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
    body.game_id = Some(id);
    body.challenge_id = Some(challenge_id);

    let game_challenge = crate::model::game_challenge::ActiveModel::from(body)
        .update(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(game_challenge),
        })),
    ));
}

pub async fn delete_challenge(
    Path((id, challenge_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::game_challenge::Entity::delete_many()
        .filter(crate::model::game_challenge::Column::GameId.eq(id))
        .filter(crate::model::game_challenge::Column::ChallengeId.eq(challenge_id))
        .exec(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn get_team(
    Query(params): Query<crate::model::game_team::request::FindRequest>,
) -> Result<impl IntoResponse, WebError> {
    let (game_teams, total) = crate::model::game_team::find(params.game_id, params.team_id)
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(game_teams),
            "total": total,
        })),
    ));
}

pub async fn create_team(
    Json(body): Json<crate::model::game_team::request::CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let game_team = crate::model::game_team::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(game_team),
        })),
    ));
}

pub async fn update_team(
    Path((id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<crate::model::game_team::request::UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
    body.game_id = Some(id);
    body.team_id = Some(team_id);

    let game_team = crate::model::game_team::ActiveModel::from(body)
        .update(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(game_team),
        })),
    ));
}

pub async fn delete_team(
    Path((id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::game_team::Entity::delete_many()
        .filter(crate::model::game_team::Column::GameId.eq(id))
        .filter(crate::model::game_team::Column::TeamId.eq(team_id))
        .exec(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn get_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn create_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn update_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn delete_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

/// get submissions by game id will calculate rank, pts of each submissoin.
pub async fn get_submission(
    Path(id): Path<i64>, Query(params): Query<GetSubmissionRequest>,
) -> Result<impl IntoResponse, WebError> {
    let submissions = crate::model::submission::get_game_submission_model(id, params.status)
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(submissions),
        })),
    ));
}

pub async fn find_poster(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder().body(Body::from(buffer)).unwrap());
        }
        None => return Err(WebError::NotFound(String::new())),
    }
}

pub async fn save_poster(
    Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let mime: Mime = content_type.parse().unwrap();
            if mime.type_() != mime::IMAGE {
                return Err(WebError::BadRequest(String::from("forbidden_file_type")));
            }
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(WebError::BadRequest(String::from("size_too_large")));
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    let _ = crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete_poster(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

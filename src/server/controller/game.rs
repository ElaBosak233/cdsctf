use crate::{server::service, traits::Ext};
use axum::{
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use serde_json::json;

pub async fn find(Extension(ext): Extension<Ext>, Query(params): Query<crate::model::game::request::FindRequest>) -> impl IntoResponse {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && !params.is_enabled.unwrap_or(true) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
            })),
        );
    }

    match service::game::find(params).await {
        Ok((challenges, total)) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": json!(challenges),
                    "total": total,
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn create(Json(body): Json<crate::model::game::request::CreateRequest>) -> impl IntoResponse {
    match service::game::create(body).await {
        Ok(challenge) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": json!(challenge),
                })),
            )
        }
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                    "msg": format!("{:?}", e),
                })),
            )
        }
    }
}

pub async fn update(Path(id): Path<i64>, Json(mut body): Json<crate::model::game::request::UpdateRequest>) -> impl IntoResponse {
    body.id = Some(id);
    match service::game::update(body).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn delete(Path(id): Path<i64>) -> impl IntoResponse {
    match service::game::delete(id).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16()
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn find_challenge(Query(params): Query<crate::model::game_challenge::request::FindRequest>) -> impl IntoResponse {
    match service::game_challenge::find(params).await {
        Ok((game_challenges, total)) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": json!(game_challenges),
                    "total": total,
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn create_challenge(Json(body): Json<crate::model::game_challenge::request::CreateRequest>) -> impl IntoResponse {
    match service::game_challenge::create(body).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn update_challenge(
    Path((id, challenge_id)): Path<(i64, i64)>, Json(mut body): Json<crate::model::game_challenge::request::UpdateRequest>,
) -> impl IntoResponse {
    body.game_id = Some(id);
    body.challenge_id = Some(challenge_id);
    match service::game_challenge::update(body).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn delete_challenge(Path((id, challenge_id)): Path<(i64, i64)>) -> impl IntoResponse {
    match service::game_challenge::delete(id, challenge_id).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn find_team(Query(params): Query<crate::model::game_team::request::FindRequest>) -> impl IntoResponse {
    match service::game_team::find(params).await {
        Ok((game_teams, total)) => {
            return (
                StatusCode::OK,
                Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(game_teams),
                "total": total,
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn create_team(Json(body): Json<crate::model::game_team::request::CreateRequest>) -> impl IntoResponse {
    match service::game_team::create(body).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn update_team(Path((id, team_id)): Path<(i64, i64)>, Json(mut body): Json<crate::model::game_team::request::UpdateRequest>) -> impl IntoResponse {
    body.game_id = Some(id);
    body.team_id = Some(team_id);
    match service::game_team::update(body).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn delete_team(Path((id, team_id)): Path<(i64, i64)>) -> impl IntoResponse {
    match service::game_team::delete(id, team_id).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn find_notice() -> impl IntoResponse {
    todo!()
}

pub async fn create_notice() -> impl IntoResponse {
    todo!()
}

pub async fn update_notice() -> impl IntoResponse {
    todo!()
}

pub async fn delete_notice() -> impl IntoResponse {
    todo!()
}

pub async fn find_poster(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Response::builder().body(buffer.into()).unwrap();
        }
        None => return (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn save_poster(Path(id): Path<i64>, mut multipart: Multipart) -> impl IntoResponse {
    let path = format!("games/{}/poster", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let mime: Mime = content_type.parse().unwrap();
            if mime.type_() != mime::IMAGE {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "code": StatusCode::BAD_REQUEST.as_u16(),
                        "msg": "forbidden_file_type",
                    })),
                );
            }
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "code": StatusCode::BAD_REQUEST.as_u16(),
                            "msg": "size_too_large",
                        })),
                    );
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    match crate::media::save(path, filename, data).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                })),
            )
        }
    }
}

pub async fn delete_poster(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("games/{}/poster", id);

    match crate::media::delete(path).await {
        Ok(_) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                })),
            )
        }
        Err(_err) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "code": StatusCode::NOT_FOUND.as_u16(),
                })),
            )
        }
    }
}

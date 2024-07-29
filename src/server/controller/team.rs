use crate::{
    server::service::{team as team_service, user_team as user_team_service},
    traits::Ext,
};
use axum::{
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use serde_json::json;

fn can_modify_team(user: crate::model::user::Model, team_id: i64) -> bool {
    return user.group == "admin" || user.teams.iter().any(|team| team.id == team_id && team.captain_id == user.id);
}

pub async fn find(Query(params): Query<crate::model::team::request::FindRequest>) -> impl IntoResponse {
    match team_service::find(params).await {
        Ok((teams, total)) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(teams),
                "total": total,
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn create(Json(body): Json<crate::model::team::request::CreateRequest>) -> impl IntoResponse {
    match team_service::create(body).await {
        Ok(team) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(team),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, Json(mut body): Json<crate::model::team::request::UpdateRequest>,
) -> impl IntoResponse {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": "forbidden",
            })),
        );
    }
    body.id = Some(id);
    match team_service::update(body).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn delete(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> impl IntoResponse {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": "forbidden",
            })),
        );
    }
    match team_service::delete(id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn create_user(Json(body): Json<crate::model::user_team::request::CreateRequest>) -> impl IntoResponse {
    match user_team_service::create(body).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn delete_user(Path((id, user_id)): Path<(i64, i64)>) -> impl IntoResponse {
    match user_team_service::delete(user_id, id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "msg": format!("{:?}", e),
            })),
        ),
    }
}

pub async fn get_invite_token(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> impl IntoResponse {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": "forbidden",
            })),
        );
    }
    match team_service::get_invite_token(id).await {
        Ok(token) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "token": token,
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

pub async fn update_invite_token(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> impl IntoResponse {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": "forbidden",
            })),
        );
    }
    match team_service::update_invite_token(id).await {
        Ok(token) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "token": token,
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

pub async fn join(Json(body): Json<crate::model::user_team::request::JoinRequest>) -> impl IntoResponse {
    match user_team_service::join(body).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
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

pub async fn leave() -> impl IntoResponse {
    todo!()
}

pub async fn find_avatar_metadata(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("teams/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": {
                        "filename": filename,
                        "size": size,
                    },
                })),
            )
        }
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                        "code": StatusCode::NOT_FOUND.as_u16(),
                })),
            )
        }
    }
}

pub async fn find_avatar(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("teams/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Response::builder().body(buffer.into()).unwrap();
        }
        None => return (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn save_avatar(Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart) -> impl IntoResponse {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": "forbidden",
            })),
        );
    }

    let path = format!("teams/{}/avatar", id);
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

pub async fn delete_avatar(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> impl IntoResponse {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": "forbidden",
            })),
        );
    }

    let path = format!("teams/{}/avatar", id);

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

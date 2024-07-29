use axum::{
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use serde_json::json;

use crate::{server::service::user as user_service, traits::Ext, util::validate};

/// **Find** can find user's information.
///
/// ## Arguments
/// - `params`: user's information.
///
/// ## Returns
/// - `200`: find successfully.
/// - `400`: find failed.
pub async fn find(Query(params): Query<crate::model::user::request::FindRequest>) -> impl IntoResponse {
    match user_service::find(params).await {
        Ok((users, total)) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(users),
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

/// **Create** can create a new user.
///
/// ## Arguments
/// - `body`: user's information(validated).
///
/// ## Returns
/// - `200`: create successfully.
/// - `400`: create failed.
pub async fn create(validate::Json(body): validate::Json<crate::model::user::request::CreateRequest>) -> impl IntoResponse {
    match user_service::create(body).await {
        Ok(user) => (
            StatusCode::OK,
            Json(json!({
                "code": StatusCode::OK.as_u16(),
                "data": json!(user),
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

/// **Update** can update user's information.
///
/// ## Arguments
/// - `id`: user's id.
/// - `body`: user's information(validated).
/// - `ext`: extension from middleware.
///
/// ## Returns
/// - `200`: update successfully.
/// - `400`: update failed.
///
/// There are some restrictions:
/// - If operator's group is "admin", operator can update any user's information.
/// - If operator's group is not "admin", operator can update his own information, but can not update group.
pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, validate::Json(mut body): validate::Json<crate::model::user::request::UpdateRequest>,
) -> impl IntoResponse {
    let operator = ext.clone().operator.unwrap();
    body.id = Some(id);
    if operator.group == "admin"
        || (operator.id == body.id.unwrap_or(0) && (body.group.clone().is_none() || operator.group == body.group.clone().unwrap_or("".to_string())))
    {
        match user_service::update(body).await {
            Ok(()) => {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "code": StatusCode::OK.as_u16()
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
    } else {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "msg": format!("{:?}", "forbidden"),
            })),
        );
    }
}

/// **Delete** can be used to delete user.
///
/// ## Arguments
/// - `id`: user's id.
///
/// ## Returns
/// - `200`: delete successfully.
/// - `400`: delete failed.
pub async fn delete(Path(id): Path<i64>) -> impl IntoResponse {
    match user_service::delete(id).await {
        Ok(()) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16()
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

/// **Login** can be used to login with username and password.
///
/// ## Arguments
/// - `body`: username and password.
///
/// ## Returns
/// - `200`: login successfully, with token and user information.
/// - `400`: login failed.
pub async fn login(Json(body): Json<crate::model::user::request::LoginRequest>) -> impl IntoResponse {
    match user_service::login(body).await {
        Ok((user, token)) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": json!(user),
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

/// **Register** can be used to register with username, nickname, email and password.
///
/// ## Arguments
/// - `body`: username, nickname, email and password.
///
/// ## Returns
/// - `200`: register successfully, with user information.
/// - `400`: register failed.
/// - `409`: username or email has been registered.
/// - `500`: internal server error(most likely database error).
pub async fn register(validate::Json(body): validate::Json<crate::model::user::request::RegisterRequest>) -> impl IntoResponse {
    match user_service::register(body).await {
        Ok(user) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": json!(user),
                })),
            )
        }
        Err(err) => match err {
            StatusCode::CONFLICT => {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({
                        "code": StatusCode::CONFLICT.as_u16(),
                    })),
                )
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    })),
                )
            }
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "code": StatusCode::BAD_REQUEST.as_u16(),
                    })),
                )
            }
        },
    }
}

pub async fn find_avatar(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("users/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Response::builder().body(buffer.into()).unwrap();
        }
        None => return (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn find_avatar_metadata(Path(id): Path<i64>) -> impl IntoResponse {
    let path = format!("users/{}/avatar", id);
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

pub async fn save_avatar(Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart) -> impl IntoResponse {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && operator.id != id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
            })),
        );
    }

    let path = format!("users/{}/avatar", id);
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
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && operator.id != id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
            })),
        );
    }

    let path = format!("users/{}/avatar", id);

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

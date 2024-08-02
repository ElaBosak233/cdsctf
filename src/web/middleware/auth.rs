use crate::{database::get_db, model::user::request::FindRequest, web::traits::Error};
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::EntityTrait;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;

use crate::{util, web::traits::Ext};

pub fn jwt(
    group: util::jwt::Group,
) -> impl Fn(
    Request<axum::body::Body>,
    Next,
) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>>
       + Clone {
    move |mut req: Request<axum::body::Body>, next: Next| {
        Box::pin({
            let value = group.clone();
            async move {
                let token = req
                    .headers()
                    .get("Authorization")
                    .and_then(|header| header.to_str().ok())
                    // .and_then(|header| header.strip_prefix("Bearer "))
                    .unwrap_or("");

                let decoding_key =
                    DecodingKey::from_secret(util::jwt::get_secret().await.as_bytes());
                let validation = Validation::default();

                match decode::<util::jwt::Claims>(token, &decoding_key, &validation) {
                    Ok(token_data) => {
                        let result = crate::model::user::Entity::find_by_id(token_data.claims.id)
                            .one(&get_db())
                            .await;

                        if let Err(_err) = result {
                            return Ok((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                                    "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                                    "msg": "internal_server_error"
                                })),
                            )
                                .into_response());
                        }

                        let user = result.unwrap();

                        if user.is_none() {
                            return Ok((
                                StatusCode::UNAUTHORIZED,
                                Json(json!({
                                    "code": StatusCode::UNAUTHORIZED.as_u16(),
                                    "msg": "unauthorized"
                                })),
                            )
                                .into_response());
                        }

                        let user = user.unwrap();

                        req.extensions_mut().insert(Ext {
                            operator: Some(user.clone()),
                        });

                        if (value as u8)
                            <= (util::jwt::Group::from_str(user.group.clone())
                                .unwrap_or(util::jwt::Group::Banned)
                                as u8)
                        {
                            return Ok(next.run(req).await);
                        } else {
                            return Ok((
                                StatusCode::FORBIDDEN,
                                Json(json!({
                                    "code": StatusCode::FORBIDDEN.as_u16(),
                                    "msg": "forbidden"
                                })),
                            )
                                .into_response());
                        }
                    }
                    Err(_) => {
                        return Ok((
                            StatusCode::UNAUTHORIZED,
                            Json(json!({
                                "code": StatusCode::UNAUTHORIZED.as_u16(),
                                "msg": "unauthorized"
                            })),
                        )
                            .into_response());
                    }
                }
            }
        })
    }
}

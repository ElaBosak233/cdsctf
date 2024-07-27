use crate::model::user::request::FindRequest;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use std::future::Future;
use std::pin::Pin;

use crate::server::service::user as user_service;
use crate::{traits::Ext, util};

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
                        let (users, total) = user_service::find(FindRequest {
                            id: Some(token_data.claims.id),
                            ..Default::default()
                        })
                        .await
                        .unwrap();

                        if total == 0 {
                            return Ok((
                                StatusCode::UNAUTHORIZED,
                                Json(json!({
                                    "code": StatusCode::UNAUTHORIZED.as_u16(),
                                    "msg": "unauthorized"
                                })),
                            )
                                .into_response());
                        }

                        let user = users.get(0).unwrap();
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

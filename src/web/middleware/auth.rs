use std::net::SocketAddr;

use crate::{database::get_db, model::user::group::Group, web::traits::WebError};
use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::{util, web::traits::Ext};

pub async fn jwt(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        // .and_then(|header| header.strip_prefix("Bearer "))
        .unwrap_or("");

    let decoding_key = DecodingKey::from_secret(util::jwt::get_secret().await.as_bytes());
    let validation = Validation::default();

    let result = decode::<util::jwt::Claims>(token, &decoding_key, &validation);

    if let Ok(token_data) = result {
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
            return Err(WebError::NotFound(String::from("not_found")));
        }

        let user = user.unwrap();

        if user.group == Group::Banned {
            return Err(WebError::Forbidden(String::from("forbidden")));
        }

        let ConnectInfo(addr) = req.extensions().get::<ConnectInfo<SocketAddr>>().unwrap();

        let client_ip = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|header_value| header_value.to_str().ok().map(|s| s.to_string()))
            .unwrap_or_else(|| addr.ip().to_owned().to_string());

        req.extensions_mut().insert(Ext {
            operator: Some(user.clone()),
            client_ip: client_ip,
        });
    }

    return Ok(next.run(req).await);
}

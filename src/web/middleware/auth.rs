use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    http::header::COOKIE,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::EntityTrait;

use crate::{
    db::{entity::user::Group, get_db},
    util,
    web::traits::{Ext, WebError},
};

pub async fn jwt(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let cookies = req
        .headers()
        .get(COOKIE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("")
        .to_string();

    let mut jar = cookie::CookieJar::new();
    let cookies: Vec<String> = cookies
        .split(";")
        .map(|cookie| cookie.trim().to_string())
        .collect();
    for cookie in cookies {
        if let Ok(parsed_cookie) = cookie::Cookie::parse(cookie) {
            jar.add(parsed_cookie);
        }
    }

    let token = jar.get("token").map(|cookie| cookie.value()).unwrap_or("");

    let decoding_key = DecodingKey::from_secret(util::jwt::get_secret().await.as_bytes());
    let validation = Validation::default();

    let mut user: Option<crate::shared::User> = None;

    let result = decode::<util::jwt::Claims>(token, &decoding_key, &validation);

    if let Ok(data) = result {
        user = crate::db::entity::user::Entity::find_by_id(data.claims.id)
            .one(get_db())
            .await?
            .map(|user| user.into());

        if user.is_none() {
            return Err(WebError::Unauthorized(String::from("not_found")));
        }

        let user = user.clone().unwrap();

        if user.group == Group::Banned {
            return Err(WebError::Forbidden(String::from("forbidden")));
        }
    }

    let ConnectInfo(addr) = req.extensions().get::<ConnectInfo<SocketAddr>>().unwrap();

    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|header_value| header_value.to_str().ok().map(|s| s.to_string()))
        .unwrap_or_else(|| addr.ip().to_owned().to_string());

    req.extensions_mut().insert(Ext {
        operator: user,
        client_ip,
    });

    Ok(next.run(req).await)
}

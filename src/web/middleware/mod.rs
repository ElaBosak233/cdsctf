pub mod error;

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
use serde_json::json;

use crate::{
    db::{entity::user::Group, get_db},
    web,
    web::traits::{Ext, WebError},
};

pub async fn auth(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let mut ext = req
        .extensions()
        .get::<Ext>()
        .unwrap_or(&Ext::default())
        .to_owned();

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

    let decoding_key = DecodingKey::from_secret(web::util::jwt::get_secret().await.as_bytes());
    let validation = Validation::default();

    let mut user: Option<crate::db::transfer::User> = None;

    if let Ok(data) = decode::<web::util::jwt::Claims>(token, &decoding_key, &validation) {
        user = crate::db::entity::user::Entity::find_by_id(data.claims.id)
            .one(get_db())
            .await?
            .map(|user| user.into());

        if user.is_none() {
            return Err(WebError::Unauthorized(json!("not_found")));
        }

        let user = user.clone().unwrap();

        if user.group == Group::Banned {
            return Err(WebError::Forbidden(json!("forbidden")));
        }
    }

    ext.operator = user;

    req.extensions_mut().insert(ext);

    Ok(next.run(req).await)
}

pub async fn network(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let mut ext = req
        .extensions()
        .get::<Ext>()
        .unwrap_or(&Ext::default())
        .to_owned();

    let ConnectInfo(addr) = req.extensions().get::<ConnectInfo<SocketAddr>>().unwrap();

    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|header_value| header_value.to_str().ok().map(|s| s.to_string()))
        .unwrap_or_else(|| addr.ip().to_owned().to_string());

    ext.client_ip = client_ip;

    req.extensions_mut().insert(ext);

    Ok(next.run(req).await)
}

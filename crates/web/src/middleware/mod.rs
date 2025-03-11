pub mod error;
pub mod network;

use axum::{
    body::Body, extract::Request, http::header::COOKIE, middleware::Next, response::Response,
};
use cds_db::{entity::user::Group, get_db};
use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

use crate::traits::{Ext, WebError};

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

    let decoding_key = DecodingKey::from_secret(cds_config::get_constant().jwt.secret.as_bytes());
    let validation = Validation::default();

    if let Ok(data) = decode::<crate::util::jwt::Claims>(token, &decoding_key, &validation) {
        if let Some(user) = cds_db::entity::user::Entity::find()
            .filter(cds_db::entity::user::Column::Id.eq(data.claims.id))
            .filter(cds_db::entity::user::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .map(|user| cds_db::transfer::User::from(user))
        {
            if user.group == Group::Banned {
                return Err(WebError::Forbidden(json!("forbidden")));
            }

            ext.operator = Some(user);
        }
    }

    req.extensions_mut().insert(ext);

    Ok(next.run(req).await)
}

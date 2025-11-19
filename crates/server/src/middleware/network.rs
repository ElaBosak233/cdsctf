use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, HeaderValue, header::HOST},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_governor::{GovernorError, key_extractor::KeyExtractor};

use crate::{
    traits::{AuthPrincipal, WebError},
    util::network::get_client_ip,
};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct GovernorKeyExtractor;

impl KeyExtractor for GovernorKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        let ip = get_client_ip(req).ok_or(GovernorError::UnableToExtractKey)?;
        Ok(ip.to_string())
    }
}

pub async fn ip_record(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let mut ext = req
        .extensions()
        .get::<AuthPrincipal>()
        .unwrap_or(&AuthPrincipal::default())
        .to_owned();

    let client_ip = get_client_ip(&req);

    match client_ip {
        Some(client_ip) => {
            ext.client_ip = client_ip.to_string();
        }
        _ => {
            return Err(WebError::BadRequest(json!("ip_extract_failed")));
        }
    }

    req.extensions_mut().insert(ext);

    Ok(next.run(req).await)
}

pub async fn real_host(mut req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let headers = req.headers().clone();

    if let Some(x_forwarded_host) = headers.get("x-forwarded-host") {
        if let Ok(host_str) = x_forwarded_host.to_str() {
            let mut new_headers = HeaderMap::new();
            for (key, value) in headers.iter() {
                new_headers.insert(key, value.clone());
            }
            new_headers.insert(
                HOST,
                HeaderValue::from_str(host_str)
                    .map_err(|_| WebError::BadRequest(json!("host_extract_failed")))?,
            );
            *req.headers_mut() = new_headers;
        }
    }

    Ok(next.run(req).await)
}

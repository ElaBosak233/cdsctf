use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_governor::{GovernorError, key_extractor::KeyExtractor};

use crate::{
    traits::{Ext, WebError},
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
        .get::<Ext>()
        .unwrap_or(&Ext::default())
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

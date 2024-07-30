use async_trait::async_trait;
use reqwest::Client;
use serde::Serialize;

use super::traits::Captcha;

#[derive(Clone)]
pub struct Turnstile {
    url: String,
    secret_key: String,
}

#[derive(Serialize)]
struct TurnstileRequest {
    #[serde(rename = "secret")]
    secret_key: String,
    #[serde(rename = "response")]
    response: String,
    #[serde(rename = "remoteip")]
    remote_ip: String,
}

impl Turnstile {
    pub fn new() -> Self {
        return Turnstile {
            url: crate::config::get_config()
                .captcha
                .turnstile
                .url
                .clone(),
            secret_key: crate::config::get_config()
                .captcha
                .turnstile
                .secret_key
                .clone(),
        };
    }
}

#[async_trait]
impl Captcha for Turnstile {
    async fn verify(&self, token: String, client_ip: String) -> bool {
        let request_body = TurnstileRequest {
            secret_key: self.secret_key.clone(),
            response: token,
            remote_ip: client_ip,
        };

        let client = Client::new();
        let resp = client
            .post(&self.url)
            .json(&request_body)
            .send()
            .await
            .unwrap();

        let response: serde_json::Value = resp.json().await.unwrap();

        match response.get("success") {
            Some(success) => return success.as_bool().unwrap(),
            None => return false,
        }
    }
}

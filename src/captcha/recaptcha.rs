use reqwest::Client;
use serde::Serialize;

use super::traits::ICaptcha;

pub struct Recaptcha {
    secret_key: String,
    url: String,
    threshold: f64,
}

#[derive(Serialize)]
struct RecaptchaRequest {
    #[serde(rename = "secret")]
    secret_key: String,
    #[serde(rename = "response")]
    response: String,
    #[serde(rename = "remoteip")]
    remote_ip: String,
}

impl ICaptcha for Recaptcha {
    fn new() -> Self {
        return Recaptcha {
            url: crate::config::get_app_config()
                .captcha
                .recaptcha
                .url
                .clone(),
            secret_key: crate::config::get_app_config()
                .captcha
                .recaptcha
                .secret_key
                .clone(),
            threshold: crate::config::get_app_config().captcha.recaptcha.threshold,
        };
    }

    async fn verify(&self, token: String, client_ip: String) -> bool {
        let request_body = RecaptchaRequest {
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
            Some(result) => {
                if let serde_json::Value::Bool(success) = result {
                    match response.get("score") {
                        Some(score) => {
                            if let serde_json::Value::Number(score) = score {
                                let score = score.as_f64().unwrap_or(0.0);
                                if *success && score >= self.threshold {
                                    return true;
                                } else {
                                    return false;
                                }
                            }
                            return false;
                        }
                        None => return false,
                    }
                }
                return false;
            }
            None => return false,
        }
    }
}

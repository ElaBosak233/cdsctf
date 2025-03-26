use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::traits::{Answer, CaptchaError};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct HCaptchaRequest {
    secret: String,
    response: String,
    #[serde(rename = "remoteip")]
    remote_ip: Option<String>,
    #[serde(rename = "sitekey")]
    site_key: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct HCaptchaResponse {
    success: bool,
    challenge_ts: Option<DateTime<Utc>>,
    hostname: Option<String>,
    credit: Option<bool>,
    #[serde(rename = "error-codes", default)]
    error_codes: Vec<String>,
    score: Option<f64>,
    #[serde(default)]
    score_reason: Vec<String>,
}

pub(crate) async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    let client = reqwest::Client::new();
    let url = &cds_config::get_variable().captcha.turnstile.url;
    let response = client
        .post(url)
        .json(&HCaptchaRequest {
            secret: cds_config::get_variable()
                .captcha
                .hcaptcha
                .secret_key
                .clone(),
            response: answer.content.clone(),
            remote_ip: answer.client_ip.clone(),
            site_key: Some(cds_config::get_variable().captcha.hcaptcha.site_key.clone()),
        })
        .send()
        .await?
        .json::<HCaptchaResponse>()
        .await?;

    debug!("{:?}", response);

    if let (Some(expected_score), Some(score)) = (
        cds_config::get_variable().captcha.hcaptcha.score,
        response.score,
    ) {
        if score < expected_score {
            return Ok(false);
        }
    }

    Ok(response.success)
}

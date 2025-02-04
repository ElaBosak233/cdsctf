use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use crate::traits::{Answer, CaptchaError};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct TurnstileRequest {
    secret: String,
    response: String,
    #[serde(rename = "clientip")]
    client_ip: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct TurnstileResponse {
    success: bool,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    #[serde(rename = "error-codes")]
    error_codes: Vec<String>,
}

pub(crate) async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    let client = reqwest::Client::new();
    let url = &cds_config::get_config().captcha.turnstile.url;
    let response = client
        .post(url)
        .json(&TurnstileRequest {
            secret: cds_config::get_config()
                .captcha
                .turnstile
                .secret_key
                .clone(),
            response: answer.content.clone(),
            client_ip: answer.client_ip.clone(),
        })
        .send()
        .await?
        .json::<TurnstileResponse>()
        .await?;


    debug!("{:?}", response);

    Ok(response.success)
}

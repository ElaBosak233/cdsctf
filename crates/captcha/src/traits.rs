use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Captcha {
    pub id: String,
    pub challenge: String,
    pub criteria: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Answer {
    pub id: Option<String>,
    pub content: String,
    pub client_ip: Option<String>,
}

#[derive(Debug, Error)]
pub enum CaptchaError {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}

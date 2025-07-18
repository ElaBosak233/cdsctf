use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::traits::MediaError;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailType {
    Verify,
    Forget,
}

impl EmailType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmailType::Verify => "verify",
            EmailType::Forget => "forget",
        }
    }
}

impl Display for EmailType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub async fn get_email(email_type: EmailType) -> Result<String, MediaError> {
    let data = crate::get("configs/emails".to_owned(), format!("{email_type}.html")).await?;
    Ok(String::from_utf8_lossy(&data).parse().unwrap())
}

pub async fn save_email(email_type: EmailType, content: String) -> Result<(), MediaError> {
    crate::save(
        "configs/emails".to_owned(),
        format!("{email_type}.html"),
        content.as_bytes().to_vec(),
    )
    .await
}

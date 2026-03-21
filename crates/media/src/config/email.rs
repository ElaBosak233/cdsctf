use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{Media, traits::MediaError};

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
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

#[derive(Clone)]
pub struct Email<'a> {
    pub media: &'a Media,
}

impl<'a> Email<'a> {
    pub(crate) fn new(media: &'a Media) -> Self {
        Self { media }
    }

    pub async fn get_email(&self, email_type: EmailType) -> Result<String, MediaError> {
        let data = self
            .media
            .get("configs/emails".to_owned(), format!("{email_type}.html"))
            .await?;
        Ok(String::from_utf8_lossy(&data).parse().unwrap_or_default())
    }

    pub async fn save_email(
        &self,
        email_type: EmailType,
        content: String,
    ) -> Result<(), MediaError> {
        self.media
            .save(
                "configs/emails".to_owned(),
                format!("{email_type}.html"),
                content.as_bytes().to_vec(),
            )
            .await
    }
}

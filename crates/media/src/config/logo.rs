//! Object storage / media — `logo` (S3 and related helpers).

use crate::{Media, traits::MediaError};

#[derive(Clone)]
pub struct Logo<'a> {
    media: &'a Media,
}

impl<'a> Logo<'a> {
    /// Constructs a new value.
    pub(crate) fn new(media: &'a Media) -> Self {
        Self { media }
    }

    /// Returns logo.

    pub async fn get_logo(&self) -> Result<Vec<u8>, MediaError> {
        self.media
            .get("configs".to_owned(), "logo".to_owned())
            .await
    }
}

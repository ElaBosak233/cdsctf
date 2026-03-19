use crate::{Media, traits::MediaError};

#[derive(Clone)]
pub struct Logo<'a> {
    media: &'a Media,
}

impl<'a> Logo<'a> {
    pub(crate) fn new(media: &'a Media) -> Self {
        Self { media }
    }

    pub async fn get_logo(&self) -> Result<Vec<u8>, MediaError> {
        self.media
            .get("configs".to_owned(), "logo".to_owned())
            .await
    }
}

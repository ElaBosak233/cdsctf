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
        let path = "configs/logo".to_owned();
        match self.media.scan_dir(path.clone()).await?.first() {
            Some((filename, _size)) => {
                let buffer = self.media.get(path, filename.to_string()).await?;
                Ok(buffer)
            }
            None => Err(MediaError::NotFound(path)),
        }
    }
}

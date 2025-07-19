use crate::traits::MediaError;

pub async fn get_logo() -> Result<Vec<u8>, MediaError> {
    let path = "configs/logo".to_owned();
    match crate::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = crate::get(path, filename.to_string()).await?;
            Ok(buffer)
        }
        None => Err(MediaError::NotFound(path)),
    }
}

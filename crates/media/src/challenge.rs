use std::path::PathBuf;

use uuid::Uuid;

use crate::traits::MediaError;

pub async fn get_root_path(challenge_id: &Uuid) -> Result<PathBuf, MediaError> {
    let filepath = PathBuf::from(&cds_env::get_constant().media.path)
        .join(format!("challenges/{}", challenge_id));

    if !filepath.exists() {
        tokio::fs::create_dir_all(&filepath).await?;
    }

    Ok(filepath)
}

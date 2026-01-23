use std::path::PathBuf;

use cds_env::Env;

use crate::traits::MediaError;

pub async fn get_root_path(env: &Env, challenge_id: i64) -> Result<PathBuf, MediaError> {
    let filepath = PathBuf::from(&env.media.path)
        .join("challenges")
        .join(challenge_id.to_string());

    if !filepath.exists() {
        tokio::fs::create_dir_all(&filepath).await?;
    }

    Ok(filepath)
}

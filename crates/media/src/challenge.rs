use std::path::PathBuf;

use cds_env::Env;

use crate::traits::MediaError;

pub async fn get_root_path(_env: &Env, challenge_id: i64) -> Result<PathBuf, MediaError> {
    Ok(PathBuf::from("challenges").join(challenge_id.to_string()))
}

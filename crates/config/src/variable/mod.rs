pub mod auth;
pub mod captcha;
pub mod meta;

use std::path::Path;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, metadata};

use crate::traits::ConfigError;

static VARIABLE: OnceCell<Variable> = OnceCell::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    pub auth: auth::Config,
    pub captcha: captcha::Config,
    pub meta: meta::Config,
}

pub fn get_variable() -> &'static Variable {
    VARIABLE.get().unwrap()
}

pub async fn init() -> Result<(), ConfigError> {
    let target_path = Path::new("data/configs/variable.toml");
    if !target_path.exists() {
        if let Some(parent) = target_path.parent() {
            if metadata(parent).await.is_err() {
                create_dir_all(parent).await?;
            }
        }
        let content = cds_assets::get("configs/variable.toml").unwrap_or_default();
        tokio::fs::write(target_path, content).await?;
    }

    let content = tokio::fs::read_to_string(target_path).await?;
    VARIABLE
        .set(toml::from_str(&content)?)
        .map_err(|_| anyhow!("Failed to set variable config into OnceCell."))?;

    Ok(())
}

pub async fn save() -> Result<(), ConfigError> {
    let target_path = Path::new("data/configs/variable.toml");
    let content = toml::to_string(get_variable())?;
    tokio::fs::write(target_path, content).await?;

    Ok(())
}

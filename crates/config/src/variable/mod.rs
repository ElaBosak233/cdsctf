pub mod auth;
pub mod captcha;
pub mod meta;

use std::{path::Path, sync::RwLock};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, metadata};

use crate::traits::ConfigError;

static VARIABLE: Lazy<RwLock<Variable>> = Lazy::new(|| RwLock::new(Variable::default()));

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Variable {
    pub meta: meta::Config,
    pub auth: auth::Config,
    pub captcha: captcha::Config,
}

impl Variable {
    pub fn desensitize(&self) -> Self {
        Self {
            meta: self.meta.to_owned(),
            auth: self.auth.to_owned(),
            captcha: self.captcha.desensitize(),
        }
    }
}

pub fn get_variable() -> Variable {
    VARIABLE.read().unwrap().to_owned()
}

pub fn set_variable(variable: Variable) -> Result<(), ConfigError> {
    let mut write_guard = VARIABLE
        .write()
        .map_err(|_| anyhow!("Failed to acquire write lock on VARIABLE"))?;
    *write_guard = variable;
    Ok(())
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
    set_variable(toml::from_str(&content)?)?;

    Ok(())
}

pub async fn save() -> Result<(), ConfigError> {
    let target_path = Path::new("data/configs/variable.toml");
    let content = toml::to_string(&get_variable())?;
    tokio::fs::write(target_path, content).await?;

    Ok(())
}

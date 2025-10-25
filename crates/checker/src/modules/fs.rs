use anyhow::anyhow;
use cds_engine::{rune, rune::Module};
use ring::rand::{SecureRandom, SystemRandom};
use tracing::debug;

#[rune::module(::fs)]
pub async fn module(_stdio: bool, challenge_id: i64) -> Result<Module, anyhow::Error> {
    let mut module = Module::from_meta(module_meta)?;
    let root = cds_media::challenge::get_root_path(challenge_id).await?;

    module
        .function("key", {
            let root = root.clone();
            move || -> Result<String, anyhow::Error> {
                let full_path = root.join(".key");

                let key = if full_path.exists() {
                    std::fs::read_to_string(full_path)?
                } else {
                    debug!(challenge_id = challenge_id, "Generating new key");

                    let rng = SystemRandom::new();
                    let mut bytes = [0u8; 64];
                    rng.fill(&mut bytes).unwrap();
                    let key = hex::encode(bytes);
                    std::fs::write(full_path, format!("{}", key))?;

                    key
                };

                Ok(key)
            }
        })
        .build()?;

    module
        .function("read_to_string", {
            let root = root.clone();
            move |path: String| -> Result<String, anyhow::Error> {
                let full_path = root.join(&path).canonicalize()?;

                if !full_path.starts_with(&root) {
                    return Err(anyhow!("access_denied"));
                }

                let content = std::fs::read_to_string(&full_path)?;

                Ok(content)
            }
        })
        .build()?;

    module
        .function("write", {
            let root = root.clone();
            move |path: String, content: String| -> Result<(), anyhow::Error> {
                let full_path = root.join(&path).canonicalize()?;

                if !full_path.starts_with(&root) {
                    return Err(anyhow!("access_denied"));
                }

                std::fs::write(&full_path, content)?;

                Ok(())
            }
        })
        .build()?;

    Ok(module)
}

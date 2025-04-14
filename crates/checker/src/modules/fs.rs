use anyhow::anyhow;
use rune::{ContextError, Module};
use std::path::PathBuf;

#[rune::module(::fs)]
pub fn module(_stdio: bool, root: PathBuf) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;

    module
        .function("read_to_string", {
            let root = root.clone();
            move |path: String| -> Result<String, anyhow::Error> {
                let full_path = root
                    .join(&path)
                    .canonicalize()?;

                if !full_path.starts_with(&root) {
                    return Err(anyhow!("access_denied"));
                }

                let content = std::fs::read_to_string(&full_path)?;

                Ok(content)
            }
        }).build()?;

    module
        .function("write", {
            let root = root.clone();
            move |path: String, content: String| -> Result<(), anyhow::Error> {
                let full_path = root
                    .join(&path)
                    .canonicalize()?;

                if !full_path.starts_with(&root) {
                    return Err(anyhow!("access_denied"));
                }

                std::fs::write(&full_path, content)?;

                Ok(())
            }
        }).build()?;

    Ok(module)
}

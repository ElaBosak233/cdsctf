use anyhow::anyhow;
use cds_engine::{rune, rune::Module};
use cds_media::{Media, traits::MediaError};
use ring::rand::{SecureRandom, SystemRandom};
use tokio::runtime::Handle;
use tracing::debug;

#[rune::module(::fs)]
pub async fn module(
    _stdio: bool,
    media: Media,
    challenge_id: i64,
) -> Result<Module, anyhow::Error> {
    let mut module = Module::from_meta(module_meta)?;
    let base = format!("challenges/{}", challenge_id);

    module
        .function("key", {
            let base = base.clone();
            let media = media.clone();
            move || -> Result<String, anyhow::Error> {
                let base = base.clone();
                let media = media.clone();

                tokio::task::block_in_place(|| {
                    Handle::current().block_on(async move {
                        match media.get(base.clone(), ".key".to_string()).await {
                            Ok(data) => {
                                String::from_utf8(data).map_err(|_| anyhow!("invalid_key_encoding"))
                            }
                            Err(MediaError::NotFound(_)) => {
                                debug!(challenge_id = challenge_id, "Generating new key");

                                let rng = SystemRandom::new();
                                let mut bytes = [0u8; 64];
                                rng.fill(&mut bytes).unwrap();
                                let key = hex::encode(bytes);

                                media
                                    .save(base.clone(), ".key".to_string(), key.as_bytes().to_vec())
                                    .await?;

                                Ok(key)
                            }
                            Err(err) => Err(anyhow!(err)),
                        }
                    })
                })
            }
        })
        .build()?;

    module
        .function("read_to_string", {
            let base = base.clone();
            let media = media.clone();
            move |path: String| -> Result<String, anyhow::Error> {
                let base = base.clone();
                let media = media.clone();

                tokio::task::block_in_place(|| {
                    Handle::current().block_on(async move {
                        let data = media.get(base.clone(), path).await?;
                        String::from_utf8(data).map_err(|_| anyhow!("invalid_utf8"))
                    })
                })
            }
        })
        .build()?;

    module
        .function("write", {
            let base = base.clone();
            let media = media.clone();
            move |path: String, content: String| -> Result<(), anyhow::Error> {
                let base = base.clone();
                let media = media.clone();

                tokio::task::block_in_place(|| {
                    Handle::current().block_on(async move {
                        media
                            .save(base.clone(), path, content.into_bytes())
                            .await
                            .map_err(|err| anyhow!(err))?;
                        Ok(())
                    })
                })
            }
        })
        .build()?;

    Ok(module)
}

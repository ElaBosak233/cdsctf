//! Async Lua module `cds.fs` backed by challenge object storage.

use cds_engine::{mlua::Lua, traits::EngineError};
use cds_media::{Media, traits::MediaError};
use once_cell::sync::Lazy;
use ring::rand::{SecureRandom, SystemRandom};
use tokio::sync::Mutex;
use tracing::debug;

static KEY_CREATION_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub fn install(lua: &Lua, media: Media, challenge_id: i64) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "fs")?;
    let base = format!("challenges/{challenge_id}");

    module.set(
        "key",
        lua.create_async_function({
            let base = base.clone();
            let media = media.clone();
            move |_lua, ()| {
                let base = base.clone();
                let media = media.clone();
                async move {
                    match media.get(base.clone(), ".key".to_owned()).await {
                        Ok(data) => String::from_utf8(data).map_err(|_| {
                            mlua::Error::RuntimeError("invalid_key_encoding".to_owned())
                        }),
                        Err(MediaError::NotFound(_)) => {
                            let _guard = KEY_CREATION_LOCK.lock().await;
                            match media.get(base.clone(), ".key".to_owned()).await {
                                Ok(data) => String::from_utf8(data).map_err(|_| {
                                    mlua::Error::RuntimeError("invalid_key_encoding".to_owned())
                                }),
                                Err(MediaError::NotFound(_)) => {
                                    debug!(challenge_id, "Generating new checker key");
                                    let mut bytes = [0_u8; 64];
                                    SystemRandom::new().fill(&mut bytes).map_err(|_| {
                                        mlua::Error::RuntimeError(
                                            "key_generation_failed".to_owned(),
                                        )
                                    })?;
                                    let key = hex::encode(bytes);
                                    media
                                        .save(base, ".key".to_owned(), key.as_bytes().to_vec())
                                        .await
                                        .map_err(mlua::Error::external)?;
                                    Ok(key)
                                }
                                Err(error) => Err(mlua::Error::external(error)),
                            }
                        }
                        Err(error) => Err(mlua::Error::external(error)),
                    }
                }
            }
        })?,
    )?;

    module.set(
        "read_to_string",
        lua.create_async_function({
            let base = base.clone();
            let media = media.clone();
            move |_lua, path: String| {
                let base = base.clone();
                let media = media.clone();
                async move {
                    let data = media.get(base, path).await.map_err(mlua::Error::external)?;
                    String::from_utf8(data)
                        .map_err(|_| mlua::Error::RuntimeError("invalid_utf8".to_owned()))
                }
            }
        })?,
    )?;

    module.set(
        "write",
        lua.create_async_function(move |_lua, (path, content): (String, String)| {
            let base = base.clone();
            let media = media.clone();
            async move {
                media
                    .save(base, path, content.into_bytes())
                    .await
                    .map_err(mlua::Error::external)
            }
        })?,
    )?;
    Ok(())
}

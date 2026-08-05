//! Async Lua module `cds.fs` backed by challenge object storage.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use cds_engine::{mlua::Lua, traits::EngineError};
use cds_media::{Media, traits::MediaError};

pub(crate) type KeyCache = Arc<RwLock<HashMap<i64, String>>>;

fn decode_key(data: Vec<u8>) -> Result<String, mlua::Error> {
    let key = String::from_utf8(data)
        .map_err(|_| mlua::Error::RuntimeError("invalid_key_encoding".to_owned()))?;
    if key.len() != 128 || hex::decode(&key).map_or(true, |decoded| decoded.len() != 64) {
        return Err(mlua::Error::RuntimeError("invalid_key_encoding".to_owned()));
    }
    Ok(key)
}

fn ensure_public_path(path: &str) -> Result<(), mlua::Error> {
    if path == ".key" {
        Err(mlua::Error::RuntimeError("reserved_path".to_owned()))
    } else {
        Ok(())
    }
}

pub fn install(
    lua: &Lua,
    media: Media,
    key_cache: KeyCache,
    challenge_id: i64,
) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "fs")?;
    let base = format!("challenges/{challenge_id}");

    module.set(
        "key",
        lua.create_async_function({
            let base = base.clone();
            let media = media.clone();
            let key_cache = key_cache.clone();
            move |_lua, ()| {
                let base = base.clone();
                let media = media.clone();
                let key_cache = key_cache.clone();
                async move {
                    if let Some(key) = key_cache
                        .read()
                        .map_err(|_| {
                            mlua::Error::RuntimeError("checker_key_cache_failed".to_owned())
                        })?
                        .get(&challenge_id)
                        .cloned()
                    {
                        return Ok(key);
                    }
                    let data =
                        media
                            .get(base, ".key".to_owned())
                            .await
                            .map_err(|error| match error {
                                MediaError::NotFound(_) => {
                                    mlua::Error::RuntimeError("checker_key_missing".to_owned())
                                }
                                error => mlua::Error::external(error),
                            })?;
                    let key = decode_key(data)?;
                    key_cache
                        .write()
                        .map_err(|_| {
                            mlua::Error::RuntimeError("checker_key_cache_failed".to_owned())
                        })?
                        .insert(challenge_id, key.clone());
                    Ok(key)
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
                    ensure_public_path(&path)?;
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
                ensure_public_path(&path)?;
                media
                    .save(base, path, content.into_bytes())
                    .await
                    .map_err(mlua::Error::external)
            }
        })?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{decode_key, ensure_public_path};

    #[test]
    fn validates_checker_key_encoding() {
        let key = "11".repeat(64);
        assert_eq!(decode_key(key.clone().into_bytes()).unwrap(), key);
        assert!(decode_key(vec![0xFF]).is_err());
        assert!(decode_key(b"11".to_vec()).is_err());
        assert!(decode_key("zz".repeat(64).into_bytes()).is_err());
    }

    #[test]
    fn reserves_checker_key_path() {
        assert!(ensure_public_path(".key").is_err());
        assert!(ensure_public_path("attachment.txt").is_ok());
    }
}

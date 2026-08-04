//! Lua module `cds.crypto` for checker scripts.

use cds_engine::{mlua::Lua, traits::EngineError};
use ring::digest::{SHA256, SHA512};

pub fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "crypto")?;
    module.set(
        "sha256",
        lua.create_function(|_, value: String| Ok(sha256(&value)))?,
    )?;
    module.set(
        "sha512",
        lua.create_function(|_, value: String| Ok(sha512(&value)))?,
    )?;
    Ok(())
}

pub fn sha256(message: &str) -> String {
    let mut context = ring::digest::Context::new(&SHA256);
    context.update(message.as_bytes());
    hex::encode(context.finish().as_ref())
}

pub fn sha512(message: &str) -> String {
    let mut context = ring::digest::Context::new(&SHA512);
    context.update(message.as_bytes());
    hex::encode(context.finish().as_ref())
}

//! Global Lua module `crypto`.

use mlua::Lua;
use ring::digest::{SHA256, SHA512};

use crate::{global_module, traits::EngineError};

pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = global_module(lua, "crypto")?;
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

#[cfg(test)]
mod tests {
    use super::{sha256, sha512};

    #[test]
    fn hashes_strings() {
        assert_eq!(sha256("answer").len(), 64);
        assert_eq!(sha512("answer").len(), 128);
        assert_ne!(sha256("answer"), sha256("different"));
    }
}

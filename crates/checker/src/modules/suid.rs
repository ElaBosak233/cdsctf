//! Lua module `cds.suid` for checker scripts.

use cds_engine::{mlua::Lua, traits::EngineError};

pub fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "suid")?;
    module.set(
        "encode",
        lua.create_function(|_, (data, key, hyphenated): (i64, String, bool)| {
            Ok(encode(data, &key, hyphenated))
        })?,
    )?;
    module.set(
        "decode",
        lua.create_function(|_, (payload, key): (String, String)| {
            decode(&payload, &key).map_err(mlua::Error::external)
        })?,
    )?;
    Ok(())
}

pub fn encode(data: i64, key: &str, hyphenated: bool) -> String {
    crate::util::suid::encode(data, key, hyphenated)
}

pub fn decode(payload: &str, key: &str) -> Result<i64, anyhow::Error> {
    crate::util::suid::decode(payload, key)
}

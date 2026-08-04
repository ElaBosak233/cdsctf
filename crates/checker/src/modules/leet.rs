//! Lua module `cds.leet` for checker scripts.

use cds_engine::{mlua::Lua, traits::EngineError};

pub fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "leet")?;
    module.set(
        "encode",
        lua.create_function(|_, (template, data, key): (String, i64, String)| {
            Ok(encode(&template, data, &key))
        })?,
    )?;
    module.set(
        "decode",
        lua.create_function(|_, (template, payload, key): (String, String, String)| {
            decode(&template, &payload, &key).map_err(mlua::Error::external)
        })?,
    )?;
    Ok(())
}

pub fn encode(template: &str, data: i64, key: &str) -> String {
    crate::util::leet::encode(template, data, key)
}

pub fn decode(template: &str, payload: &str, key: &str) -> Result<i64, std::io::Error> {
    crate::util::leet::decode(template, payload, key)
}

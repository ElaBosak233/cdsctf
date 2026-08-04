//! Lua module `cds.regex` for checker scripts.

use cds_engine::{mlua::Lua, traits::EngineError};

pub fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "regex")?;
    module.set(
        "is_match",
        lua.create_function(|_, (pattern, value): (String, String)| {
            Ok(is_match(&pattern, &value))
        })?,
    )?;
    Ok(())
}

pub fn is_match(pattern: &str, payload: &str) -> bool {
    regex::Regex::new(pattern)
        .map(|regex| regex.is_match(payload))
        .unwrap_or(false)
}

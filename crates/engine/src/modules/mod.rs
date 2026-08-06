//! Global libraries available to every Lua script context.

pub mod crypto;
pub mod http;
pub mod json;
pub mod regex;

use mlua::Lua;

use crate::traits::EngineError;

pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    crypto::install(lua)?;
    http::install(lua)?;
    json::install(lua)?;
    regex::install(lua)?;
    Ok(())
}

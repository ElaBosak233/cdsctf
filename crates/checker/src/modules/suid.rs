//! Lua module `checker.suid` for checker scripts.

use cds_engine::{
    mlua::{Lua, Table},
    traits::EngineError,
};

pub fn install(lua: &Lua, default_key: Option<String>) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "checker", "suid")?;
    let encode_key = default_key.clone();
    module.set(
        "encode",
        lua.create_function(move |_, (data, options): (i64, Option<Table>)| {
            let (key, hyphenated) = resolve_options(options, encode_key.as_deref())?;
            Ok(encode(data, &key, hyphenated))
        })?,
    )?;
    module.set(
        "decode",
        lua.create_function(move |_, (payload, options): (String, Option<Table>)| {
            let (key, _) = resolve_options(options, default_key.as_deref())?;
            decode(&payload, &key).map_err(mlua::Error::external)
        })?,
    )?;
    Ok(())
}

fn resolve_options(
    options: Option<Table>,
    default_key: Option<&str>,
) -> mlua::Result<(String, bool)> {
    let Some(options) = options else {
        let key = default_key
            .filter(|key| !key.is_empty())
            .ok_or_else(|| mlua::Error::RuntimeError("checker_key_unavailable".to_owned()))?;
        return Ok((key.to_owned(), false));
    };
    let key = options
        .get::<Option<String>>("key")?
        .or_else(|| default_key.map(str::to_owned))
        .ok_or_else(|| mlua::Error::RuntimeError("checker_key_unavailable".to_owned()))?;
    if key.is_empty() {
        return Err(mlua::Error::RuntimeError(
            "checker_custom_key_empty".to_owned(),
        ));
    }
    Ok((key, options.get("hyphenated")?))
}

pub fn encode(data: i64, key: &str, hyphenated: bool) -> String {
    crate::util::suid::encode(data, key, hyphenated)
}

pub fn decode(payload: &str, key: &str) -> Result<i64, anyhow::Error> {
    crate::util::suid::decode(payload, key)
}

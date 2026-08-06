//! Lua module `checker.leet` for checker scripts.

use cds_engine::{
    mlua::{Lua, Table},
    traits::EngineError,
};

pub fn install(lua: &Lua, default_key: Option<String>) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "checker", "leet")?;
    let encode_key = default_key.clone();
    module.set(
        "encode",
        lua.create_function(
            move |_, (template, data, options): (String, i64, Option<Table>)| {
                let key = resolve_key(options, encode_key.as_deref())?;
                Ok(encode(&template, data, &key))
            },
        )?,
    )?;
    module.set(
        "decode",
        lua.create_function(
            move |_, (template, payload, options): (String, String, Option<Table>)| {
                let key = resolve_key(options, default_key.as_deref())?;
                decode(&template, &payload, &key).map_err(mlua::Error::external)
            },
        )?,
    )?;
    Ok(())
}

fn resolve_key(options: Option<Table>, default_key: Option<&str>) -> mlua::Result<String> {
    let key = options
        .map(|options| options.get::<Option<String>>("key"))
        .transpose()?
        .flatten()
        .or_else(|| default_key.map(str::to_owned))
        .ok_or_else(|| mlua::Error::RuntimeError("checker_key_unavailable".to_owned()))?;
    if key.is_empty() {
        return Err(mlua::Error::RuntimeError(
            "checker_custom_key_empty".to_owned(),
        ));
    }
    Ok(key)
}

pub fn encode(template: &str, data: i64, key: &str) -> String {
    crate::util::leet::encode(template, data, key)
}

pub fn decode(template: &str, payload: &str, key: &str) -> Result<i64, std::io::Error> {
    crate::util::leet::decode(template, payload, key)
}

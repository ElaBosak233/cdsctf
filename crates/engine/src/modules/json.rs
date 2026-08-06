//! Global Lua module `json`.

use mlua::{ExternalResult, Lua, LuaSerdeExt, Value};

use crate::{global_module, traits::EngineError};

pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = global_module(lua, "json")?;
    module.set(
        "encode",
        lua.create_function(|lua, value: Value| {
            let value: serde_json::Value = lua.from_value(value)?;
            serde_json::to_string(&value).into_lua_err()
        })?,
    )?;
    module.set(
        "decode",
        lua.create_function(|lua, value: String| {
            let value: serde_json::Value = serde_json::from_str(&value).into_lua_err()?;
            lua.to_value(&value)
        })?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::create_lua;

    #[test]
    fn round_trips_values() {
        let lua = create_lua().unwrap();
        let value: String = lua
            .load(r#"return json.encode({ answer = 42 })"#)
            .eval()
            .unwrap();
        let decoded: HashMap<String, i64> = serde_json::from_str(&value).unwrap();
        assert_eq!(decoded["answer"], 42);
    }
}

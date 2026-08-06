//! Global Lua module `regex`.

use mlua::Lua;

use crate::{global_module, traits::EngineError};

pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = global_module(lua, "regex")?;
    module.set(
        "is_match",
        lua.create_function(|_, (pattern, value): (String, String)| {
            Ok(is_match(&pattern, &value))
        })?,
    )?;
    Ok(())
}

pub fn is_match(pattern: &str, value: &str) -> bool {
    regex::Regex::new(pattern)
        .map(|regex| regex.is_match(value))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::is_match;

    #[test]
    fn matches_valid_patterns_and_rejects_invalid_patterns() {
        assert!(is_match("^ans", "answer"));
        assert!(!is_match("^flag$", "answer"));
        assert!(!is_match("[", "answer"));
    }
}

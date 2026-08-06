//! Lua module `checker.audit` for checker status and flag helpers.

use std::io;

use cds_engine::{
    mlua::{Lua, Table},
    traits::EngineError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Correct,
    Incorrect,
    Cheat(i64),
}

#[derive(Debug, Clone)]
pub struct Flag {
    prefix: String,
    content: String,
}

impl Flag {
    pub fn parse(value: &str) -> Result<Self, io::Error> {
        let value = value.trim();
        let prefix_end = value
            .find('{')
            .ok_or_else(|| io::Error::other("flag format is incorrect"))?;
        let prefix = value[..prefix_end].to_owned();
        let content = &value[prefix_end..];
        if !(content.starts_with('{') && content.ends_with('}')) {
            return Err(io::Error::other("flag format is incorrect"));
        }
        Ok(Self {
            prefix,
            content: content[1..content.len() - 1].to_owned(),
        })
    }

    pub fn format(&self) -> String {
        format!("{}{{{}}}", self.prefix, self.content)
    }
}

fn flag_table(lua: &Lua, flag: Flag) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("prefix", flag.prefix)?;
    table.set("content", flag.content)?;
    Ok(table)
}

fn status_table(lua: &Lua, kind: &str, operator_id: Option<i64>) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", kind)?;
    if let Some(operator_id) = operator_id {
        table.set("operator_id", operator_id)?;
    }
    Ok(table)
}

pub fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "checker", "audit")?;
    module.set(
        "parse",
        lua.create_function(|lua, value: String| {
            let flag = Flag::parse(&value).map_err(mlua::Error::external)?;
            flag_table(lua, flag)
        })?,
    )?;
    module.set(
        "new",
        lua.create_function(|lua, ()| {
            flag_table(
                lua,
                Flag {
                    prefix: String::new(),
                    content: String::new(),
                },
            )
        })?,
    )?;
    module.set(
        "format",
        lua.create_function(|_, flag: Table| {
            Ok(Flag {
                prefix: flag.get("prefix")?,
                content: flag.get("content")?,
            }
            .format())
        })?,
    )?;
    module.set(
        "correct",
        lua.create_function(|lua, ()| status_table(lua, "correct", None))?,
    )?;
    module.set(
        "incorrect",
        lua.create_function(|lua, ()| status_table(lua, "incorrect", None))?,
    )?;
    module.set(
        "cheat",
        lua.create_function(|lua, operator_id: i64| status_table(lua, "cheat", Some(operator_id)))?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Flag;

    #[test]
    fn parses_and_formats_flag() {
        let flag = Flag::parse("flag{content}").unwrap();
        assert_eq!(flag.prefix, "flag");
        assert_eq!(flag.content, "content");
        assert_eq!(flag.format(), "flag{content}");
    }
}

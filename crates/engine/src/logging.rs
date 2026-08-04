//! Lua logging bridge backed by the application's `tracing` subscriber.

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use mlua::{Function, Lua, MultiValue, Table, Value};

use crate::{module, traits::EngineError};

const MAX_LOG_ENTRIES: u32 = 256;
const MAX_LOG_MESSAGE_BYTES: usize = 8 * 1024;
const TRUNCATION_SUFFIX: &str = "...[truncated]";

#[derive(Clone, Copy)]
enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

/// Installs both the namespaced `cds.log` API and the shorter global `log`
/// alias. `print` is intentionally mapped to debug for script compatibility.
pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    let budget = Arc::new(AtomicU32::new(0));
    lua.set_app_data(LogBudget(budget.clone()));
    let log = module(lua, "log")?;

    install_level(lua, &log, "debug", Level::Debug, budget.clone())?;
    install_level(lua, &log, "info", Level::Info, budget.clone())?;
    install_level(lua, &log, "warn", Level::Warn, budget.clone())?;
    install_level(lua, &log, "error", Level::Error, budget.clone())?;

    lua.globals().set("log", log.clone())?;
    lua.globals()
        .set("print", create_log_function(lua, Level::Debug, budget)?)?;
    Ok(())
}

pub(crate) fn reset_budget(lua: &Lua) {
    if let Some(budget) = lua.app_data_ref::<LogBudget>() {
        budget.0.store(0, Ordering::Relaxed);
    }
}

struct LogBudget(Arc<AtomicU32>);

fn install_level(
    lua: &Lua,
    table: &Table,
    name: &str,
    level: Level,
    budget: Arc<AtomicU32>,
) -> Result<(), EngineError> {
    table.set(name, create_log_function(lua, level, budget)?)?;
    Ok(())
}

fn create_log_function(lua: &Lua, level: Level, budget: Arc<AtomicU32>) -> mlua::Result<Function> {
    lua.create_function(move |_lua, values: MultiValue| {
        if budget
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |count| {
                (count < MAX_LOG_ENTRIES).then_some(count + 1)
            })
            .is_err()
        {
            return Ok(());
        }

        let message = values
            .iter()
            .map(format_value)
            .collect::<Vec<_>>()
            .join("\t");
        let message = truncate_message(message);

        match level {
            Level::Debug => tracing::debug!(target: "cds.lua", message = %message),
            Level::Info => tracing::info!(target: "cds.lua", message = %message),
            Level::Warn => tracing::warn!(target: "cds.lua", message = %message),
            Level::Error => tracing::error!(target: "cds.lua", message = %message),
        }
        Ok(())
    })
}

fn format_value(value: &Value) -> String {
    value
        .to_string()
        .unwrap_or_else(|_| format!("<{}>", value.type_name()))
}

fn truncate_message(mut message: String) -> String {
    if message.len() <= MAX_LOG_MESSAGE_BYTES {
        return message;
    }

    let mut end = MAX_LOG_MESSAGE_BYTES.saturating_sub(TRUNCATION_SUFFIX.len());
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message.truncate(end);
    message.push_str(TRUNCATION_SUFFIX);
    message
}

#[cfg(test)]
mod tests {
    use mlua::{Function, Lua};

    use super::{
        MAX_LOG_ENTRIES, MAX_LOG_MESSAGE_BYTES, TRUNCATION_SUFFIX, install, truncate_message,
    };

    #[test]
    fn installs_global_and_namespaced_logging() {
        let lua = Lua::new();
        lua.globals()
            .set("cds", lua.create_table().unwrap())
            .unwrap();
        install(&lua).unwrap();

        let _: Function = lua.globals().get("print").unwrap();
        let _: Function = lua.load("return log.debug").eval().unwrap();
        let _: Function = lua.load("return cds.log.info").eval().unwrap();
    }

    #[test]
    fn accepts_multiple_values_and_maps_print_to_debug() {
        let lua = Lua::new();
        lua.globals()
            .set("cds", lua.create_table().unwrap())
            .unwrap();
        install(&lua).unwrap();

        lua.load(
            r#"
                print("value", 42, true, nil)
                log.debug("debug")
                log.info("info")
                log.warn("warn")
                log.error("error")
                cds.log.info("namespaced")
            "#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn truncates_utf8_messages_within_byte_limit() {
        let message = truncate_message("中文".repeat(MAX_LOG_MESSAGE_BYTES));

        assert!(message.len() <= MAX_LOG_MESSAGE_BYTES);
        assert!(message.ends_with(TRUNCATION_SUFFIX));
        assert!(message.is_char_boundary(message.len()));
    }

    #[test]
    fn logging_budget_is_silent_after_limit() {
        let lua = Lua::new();
        lua.globals()
            .set("cds", lua.create_table().unwrap())
            .unwrap();
        install(&lua).unwrap();

        let script = format!(
            "for _ = 1, {} do log.debug('message') end",
            MAX_LOG_ENTRIES + 32
        );
        lua.load(script).exec().unwrap();
    }
}

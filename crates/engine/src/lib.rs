//! Sandboxed Lua script engine used by challenge checkers and identity
//! providers.
//!
//! Scripts are cached by source and executed in an isolated Lua state. Native
//! Platform-specific integrations install their APIs below an explicit script
//! namespace, while generic runtime libraries remain top-level globals.

pub mod modules;
pub mod traits;

mod logging;
mod util;
mod worker;

use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

use dashmap::DashMap;
pub use mlua;
use mlua::{
    Function, HookTriggers, Lua, LuaOptions, LuaSerdeExt, MultiValue, StdLib, Table, Value,
    VmState, chunk::ChunkMode,
};
use once_cell::sync::Lazy;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::debug;

use crate::traits::{DiagnosticKind, DiagnosticMarker, EngineError};

const LUA_MEMORY_LIMIT: usize = 16 * 1024 * 1024;
const LUA_CALL_TIMEOUT: Duration = Duration::from_secs(20);
const LUA_INSTRUCTION_BATCH: u32 = 10_000;
const LUA_INSTRUCTION_BATCH_LIMIT: u32 = 500;

struct EngineContext {
    script: Arc<str>,
    bytecode: Arc<[u8]>,
    created_at: OffsetDateTime,
    pool: Arc<LuaPool>,
}

struct InstructionBudget(Arc<AtomicU32>);

struct NamespaceRegistry(Mutex<HashSet<String>>);

struct LuaPool {
    slots: Mutex<Vec<Lua>>,
    permits: Arc<Semaphore>,
}

struct LuaLease {
    lua: Option<Lua>,
    pool: Arc<LuaPool>,
    _permit: OwnedSemaphorePermit,
}

impl LuaPool {
    fn new() -> Self {
        let capacity = std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1)
            .min(32);
        Self {
            slots: Mutex::new(Vec::with_capacity(capacity)),
            permits: Arc::new(Semaphore::new(capacity)),
        }
    }

    async fn checkout(self: &Arc<Self>, configure: &ConfigureLua) -> Result<LuaLease, EngineError> {
        let permit = self
            .permits
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| anyhow::anyhow!("lua pool closed"))?;
        let lua = self.slots.lock().expect("lua pool mutex poisoned").pop();
        let lua = match lua {
            Some(lua) => lua,
            None => {
                let lua = create_lua()?;
                configure(&lua)?;
                lua
            }
        };
        reset_instruction_budget(&lua);
        logging::reset_budget(&lua);
        Ok(LuaLease {
            lua: Some(lua),
            pool: self.clone(),
            _permit: permit,
        })
    }

    fn release(&self, lua: Lua) {
        self.slots
            .lock()
            .expect("lua pool mutex poisoned")
            .push(lua);
    }
}

impl LuaLease {
    fn lua(&self) -> &Lua {
        self.lua.as_ref().expect("lua lease already released")
    }

    fn discard(&mut self) {
        self.lua.take();
    }
}

impl Drop for LuaLease {
    fn drop(&mut self) {
        if let Some(lua) = self.lua.take() {
            self.pool.release(lua);
        }
    }
}

static GLOBAL_ENGINE: Lazy<Arc<DashMap<String, EngineContext>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

fn get_global_engine() -> Arc<DashMap<String, EngineContext>> {
    GLOBAL_ENGINE.clone()
}

pub type ConfigureLua = dyn Fn(&Lua) -> Result<(), EngineError> + Send + Sync;

pub async fn init() -> Result<(), EngineError> {
    worker::cleaner().await;
    Ok(())
}

/// Creates a restricted Lua state for one compilation or execution.
pub fn create_lua() -> Result<Lua, EngineError> {
    let libs = StdLib::COROUTINE | StdLib::TABLE | StdLib::STRING | StdLib::UTF8 | StdLib::MATH;
    let lua = Lua::new_with(libs, LuaOptions::default())?;
    lua.set_memory_limit(LUA_MEMORY_LIMIT)?;

    let instruction_batches = Arc::new(AtomicU32::new(0));
    lua.set_app_data(InstructionBudget(instruction_batches.clone()));
    lua.set_hook(
        HookTriggers::new().every_nth_instruction(LUA_INSTRUCTION_BATCH),
        move |lua, _debug| {
            let instruction_batches = lua
                .app_data_ref::<InstructionBudget>()
                .expect("instruction budget missing");
            if instruction_batches.0.fetch_add(1, Ordering::Relaxed) >= LUA_INSTRUCTION_BATCH_LIMIT
            {
                Err(mlua::Error::RuntimeError(
                    "script instruction limit exceeded".to_owned(),
                ))
            } else {
                Ok(VmState::Continue)
            }
        },
    )?;

    lua.set_app_data(NamespaceRegistry(Mutex::new(HashSet::new())));
    logging::install(&lua)?;
    modules::install(&lua)?;
    Ok(lua)
}

fn reset_instruction_budget(lua: &Lua) {
    if let Some(budget) = lua.app_data_ref::<InstructionBudget>() {
        budget.0.store(0, Ordering::Relaxed);
    }
}

// Reuse native API implementations while keeping script writes local to one
// call.
fn proxy_table(
    lua: &Lua,
    source: &Table,
    visited: &mut HashMap<usize, Table>,
) -> mlua::Result<Table> {
    let pointer = source.to_pointer() as usize;
    if let Some(table) = visited.get(&pointer) {
        return Ok(table.clone());
    }

    let target = lua.create_table()?;
    visited.insert(pointer, target.clone());
    for pair in source.pairs::<Value, Value>() {
        let (key, value) = pair?;
        let key = match key {
            Value::Table(table) => Value::Table(proxy_table(lua, &table, visited)?),
            value => value,
        };
        let value = match value {
            Value::Table(table) => Value::Table(proxy_table(lua, &table, visited)?),
            value => value,
        };
        target.raw_set(key, value)?;
    }
    let metatable = lua.create_table()?;
    metatable.set("__index", source.clone())?;
    metatable.set("__metatable", false)?;
    target.set_metatable(Some(metatable))?;
    Ok(target)
}

fn execution_environment(lua: &Lua) -> mlua::Result<Table> {
    let globals = lua.globals();
    let mut visited = HashMap::new();
    let environment = lua.create_table()?;

    let mut names = [
        "crypto",
        "http",
        "json",
        "coroutine",
        "math",
        "string",
        "table",
        "utf8",
        "log",
        "regex",
        "time",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if let Some(registry) = lua.app_data_ref::<NamespaceRegistry>() {
        names.extend(
            registry
                .0
                .lock()
                .expect("lua namespace registry poisoned")
                .iter()
                .cloned(),
        );
    }

    for name in names {
        if let Value::Table(table) = globals.get(name.as_str())? {
            environment.set(name.as_str(), proxy_table(lua, &table, &mut visited)?)?;
        }
    }
    environment.set("print", globals.get::<Function>("print")?)?;
    environment.set("_G", environment.clone())?;

    let metatable = lua.create_table()?;
    metatable.set("__index", globals)?;
    metatable.set("__metatable", false)?;
    environment.set_metatable(Some(metatable))?;
    Ok(environment)
}

/// Creates and installs a module table below a host-owned namespace.
pub fn module(lua: &Lua, namespace: &str, name: &str) -> Result<Table, EngineError> {
    let globals = lua.globals();
    let namespace_table = match globals.get::<Value>(namespace)? {
        Value::Nil => {
            let table = lua.create_table()?;
            globals.set(namespace, table.clone())?;
            table
        }
        Value::Table(table) => table,
        value => {
            return Err(mlua::Error::RuntimeError(format!(
                "Lua namespace `{namespace}` is not a table (got {})",
                value.type_name()
            ))
            .into());
        }
    };
    let table = lua.create_table()?;
    namespace_table.set(name, table.clone())?;
    if let Some(registry) = lua.app_data_ref::<NamespaceRegistry>() {
        registry
            .0
            .lock()
            .expect("lua namespace registry poisoned")
            .insert(namespace.to_owned());
    }
    Ok(table)
}

/// Creates and installs a module table in the script's global namespace.
pub fn global_module(lua: &Lua, name: &str) -> Result<Table, EngineError> {
    let table = lua.create_table()?;
    lua.globals().set(name, table.clone())?;
    Ok(table)
}

fn diagnostic(message: String, span: util::DiagnosticSpan) -> DiagnosticMarker {
    DiagnosticMarker {
        kind: DiagnosticKind::Error,
        message,
        start_line: span.start_line,
        start_column: span.start_column,
        end_line: span.end_line,
        end_column: span.end_column,
    }
}

fn syntax_diagnostics(script: &str, error: impl ToString) -> Vec<DiagnosticMarker> {
    let lua_message = error.to_string();
    util::syntax_diagnostics(script, &lua_message)
        .into_iter()
        .enumerate()
        .map(|(index, parsed)| {
            let message = if index == 0 {
                lua_message.clone()
            } else {
                parsed.message
            };
            diagnostic(message, parsed.span)
        })
        .collect()
}

fn runtime_diagnostic(script: &str, error: impl ToString) -> DiagnosticMarker {
    let message = error.to_string();
    let span = util::error_line_span(script, &message);
    diagnostic(message, span)
}

/// Compiles and validates a script and its required top-level functions.
pub async fn lint(
    script: impl AsRef<str>,
    required_functions: &[&str],
    configure: &ConfigureLua,
) -> Result<(), EngineError> {
    let script = script.as_ref();
    let lua = create_lua()?;
    configure(&lua)?;

    // Compile first so syntax errors are reported separately from errors caused
    // by top-level code. Both need to be surfaced as editor diagnostics.
    let function = match lua
        .load(script)
        .set_name("@script")
        .set_mode(ChunkMode::Text)
        .into_function()
    {
        Ok(function) => function,
        Err(error) => {
            return Err(EngineError::DiagnosticsError(syntax_diagnostics(
                script, error,
            )));
        }
    };
    if let Err(error) = function.call::<()>(()) {
        return Err(EngineError::DiagnosticsError(vec![runtime_diagnostic(
            script, error,
        )]));
    }

    let globals = lua.globals();
    let mut markers = Vec::new();
    for function in required_functions {
        match globals.get::<Value>(*function)? {
            Value::Function(_) => {}
            Value::Nil => markers.push(DiagnosticMarker {
                kind: DiagnosticKind::Error,
                message: format!("Missing required function: {function}"),
                start_line: 0,
                start_column: 0,
                end_line: 0,
                end_column: 0,
            }),
            value => markers.push(DiagnosticMarker {
                kind: DiagnosticKind::Error,
                message: format!(
                    "Required global {function} must be a function, got {}",
                    value.type_name()
                ),
                start_line: 0,
                start_column: 0,
                end_line: 0,
                end_column: 0,
            }),
        }
    }

    if markers.is_empty() {
        Ok(())
    } else {
        Err(EngineError::DiagnosticsError(markers))
    }
}

/// Compiles and caches a script. VM slots are reused, but every call receives
/// a fresh protected environment so script globals are not shared.
pub async fn preload(
    key: impl AsRef<str>,
    script: impl AsRef<str>,
    _last_changed_at: Option<OffsetDateTime>,
) -> Result<(), EngineError> {
    let key = key.as_ref();
    let script = script.as_ref();
    if let Some(context) = GLOBAL_ENGINE.get(key)
        && context.script.as_ref() == script
    {
        debug!(key, "Lua script is up to date, skipping preload");
        return Ok(());
    }

    let compiler = create_lua()?;
    let function = compiler
        .load(script)
        .set_name(format!("@{key}"))
        .set_mode(ChunkMode::Text)
        .into_function()?;
    let bytecode = function.dump(false);
    GLOBAL_ENGINE.insert(
        key.to_owned(),
        EngineContext {
            script: Arc::from(script.to_owned()),
            bytecode: Arc::from(bytecode),
            created_at: OffsetDateTime::now_utc(),
            pool: Arc::new(LuaPool::new()),
        },
    );
    Ok(())
}

/// Executes a cached script with native Lua arguments.
pub async fn execute<A, R>(
    key: impl AsRef<str>,
    function: &str,
    args: A,
    configure: &ConfigureLua,
) -> Result<R, EngineError>
where
    A: mlua::IntoLuaMulti + Send,
    R: DeserializeOwned, {
    let context = GLOBAL_ENGINE
        .get(key.as_ref())
        .ok_or_else(|| EngineError::MissingContext(key.as_ref().to_owned()))?;
    let bytecode = context.bytecode.clone();
    let pool = context.pool.clone();
    drop(context);

    let mut lease = pool.checkout(configure).await?;
    let environment = execution_environment(lease.lua())?;
    lease
        .lua()
        .load(bytecode.as_ref())
        .set_mode(ChunkMode::Binary)
        .set_environment(environment.clone())
        .exec()?;
    let function = environment
        .get::<Function>(function)
        .map_err(|_| EngineError::MissingFunction(function.to_owned()))?;
    let result: Result<Value, EngineError> =
        tokio::time::timeout(LUA_CALL_TIMEOUT, function.call_async(args))
            .await
            .map_err(|_| EngineError::Timeout)?
            .map_err(EngineError::from);
    if result.is_err() {
        lease.discard();
    }
    let value = result?;
    Ok(lease.lua().from_value(value)?)
}

/// Executes a function with JSON values as multiple Lua arguments.
pub async fn execute_json<R>(
    key: impl AsRef<str>,
    function: &str,
    args: &[JsonValue],
    configure: &ConfigureLua,
) -> Result<R, EngineError>
where
    R: DeserializeOwned, {
    let context = GLOBAL_ENGINE
        .get(key.as_ref())
        .ok_or_else(|| EngineError::MissingContext(key.as_ref().to_owned()))?;
    let bytecode = context.bytecode.clone();
    let pool = context.pool.clone();
    drop(context);

    let mut lease = pool.checkout(configure).await?;
    let environment = execution_environment(lease.lua())?;
    lease
        .lua()
        .load(bytecode.as_ref())
        .set_mode(ChunkMode::Binary)
        .set_environment(environment.clone())
        .exec()?;
    let function = environment
        .get::<Function>(function)
        .map_err(|_| EngineError::MissingFunction(function.to_owned()))?;
    let mut values = MultiValue::new();
    for arg in args {
        values.push_back(lease.lua().to_value(arg)?);
    }
    let result: Result<Value, EngineError> =
        tokio::time::timeout(LUA_CALL_TIMEOUT, function.call_async(values))
            .await
            .map_err(|_| EngineError::Timeout)?
            .map_err(EngineError::from);
    if result.is_err() {
        lease.discard();
    }
    let value = result?;
    Ok(lease.lua().from_value(value)?)
}

pub fn from_lua<T>(lua: &Lua, value: Value) -> Result<T, EngineError>
where
    T: serde::de::DeserializeOwned, {
    Ok(lua.from_value(value)?)
}

pub fn to_lua<T>(lua: &Lua, value: &T) -> Result<Value, EngineError>
where
    T: Serialize + ?Sized, {
    Ok(lua.to_value(value)?)
}

pub fn clear_cache() {
    GLOBAL_ENGINE.clear();
}

#[cfg(test)]
mod tests {
    use mlua::Lua;

    use super::{ConfigureLua, clear_cache, execute, lint, preload};
    use crate::traits::EngineError;

    fn configure() -> &'static ConfigureLua {
        &|_lua| Ok(())
    }

    #[tokio::test]
    async fn validates_syntax_and_required_functions() {
        let error = lint("function check(", &["check"], configure())
            .await
            .unwrap_err();
        let EngineError::DiagnosticsError(markers) = error else {
            panic!("expected diagnostics error");
        };
        assert_eq!(markers[0].start_line, 0);
        assert_eq!(markers[0].start_column, 0);
        assert_eq!(markers[0].end_column, "function check(".len());

        let error = lint("function check() end", &["check", "generate"], configure())
            .await
            .unwrap_err();
        let EngineError::DiagnosticsError(markers) = error else {
            panic!("expected diagnostics error");
        };
        assert!(markers[0].message.contains("generate"));
    }

    #[tokio::test]
    async fn rejects_unbounded_top_level_execution() {
        let error = lint("while true do end", &[], configure())
            .await
            .unwrap_err();
        let EngineError::DiagnosticsError(markers) = error else {
            panic!("expected diagnostics error");
        };
        assert!(markers[0].message.contains("instruction limit"));
    }

    #[tokio::test]
    async fn marks_runtime_errors_across_the_reported_line() {
        let error = lint(
            "local value = nil + 1\nfunction check() end",
            &[],
            configure(),
        )
        .await
        .unwrap_err();
        let EngineError::DiagnosticsError(markers) = error else {
            panic!("expected diagnostics error");
        };
        assert_eq!(markers[0].start_line, 0);
        assert_eq!(markers[0].start_column, 0);
        assert_eq!(markers[0].end_column, "local value = nil + 1".len());
    }

    #[tokio::test]
    async fn reports_multiple_syntax_errors() {
        let script = "function check()\n  if true then\n    return true\nfunction generate() end";
        let error = lint(script, &["check", "generate"], configure())
            .await
            .unwrap_err();
        let EngineError::DiagnosticsError(markers) = error else {
            panic!("expected diagnostics error");
        };

        assert_eq!(markers.len(), 2);
        assert_ne!(markers[0].message, markers[1].message);
    }

    #[tokio::test]
    async fn refreshes_cache_when_source_changes() {
        clear_cache();
        preload("test/cache", "function value() return 1 end", None)
            .await
            .unwrap();
        preload("test/cache", "function value() return 2 end", None)
            .await
            .unwrap();
        let result: i64 = execute("test/cache", "value", (), configure())
            .await
            .unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn isolates_globals_between_pooled_executions() {
        clear_cache();
        let configure: &ConfigureLua = &|lua: &Lua| {
            let state = super::module(lua, "checker", "state")?;
            state.set("value", 0)?;
            Ok(())
        };
        preload(
            "test/pool-isolation",
            r#"
                counter = (counter or 0) + 1
                checker.state.value = checker.state.value + 1
                function value()
                    return counter * 100 + checker.state.value
                end
            "#,
            None,
        )
        .await
        .unwrap();

        let first: i64 = execute("test/pool-isolation", "value", (), configure)
            .await
            .unwrap();
        let second: i64 = execute("test/pool-isolation", "value", (), configure)
            .await
            .unwrap();
        assert_eq!(first, 101);
        assert_eq!(second, 101);
    }

    #[tokio::test]
    async fn exposes_time_module_during_execution() {
        clear_cache();
        preload(
            "test/time-module",
            "function pause() time.sleep(0) return type(time.sleep) end",
            None,
        )
        .await
        .unwrap();
        let result: String = execute("test/time-module", "pause", (), configure())
            .await
            .unwrap();
        assert_eq!(result, "function");
    }
}

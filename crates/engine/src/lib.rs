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
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant},
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
// Includes execution states and short-lived compilation/lint states. Native
// HTTP/storage allocations are outside Lua's allocator limit.
const MAX_LUA_STATES: usize = 24;
const MAX_CHECKER_EXECUTIONS: usize = 16;
const MAX_OTHER_EXECUTIONS: usize = 4;
const MAX_SCRIPT_EXECUTIONS: usize = 4;
static VM_PERMITS: Lazy<Arc<Semaphore>> = Lazy::new(|| Arc::new(Semaphore::new(MAX_LUA_STATES)));
static EXECUTION_LIMITS: Lazy<ExecutionLimits> =
    Lazy::new(|| ExecutionLimits::new(MAX_CHECKER_EXECUTIONS, MAX_OTHER_EXECUTIONS));

const LUA_INSTRUCTION_BATCH: u32 = 10_000;
const LUA_INSTRUCTION_BATCH_LIMIT: u32 = 500;

struct EngineContext {
    script: Arc<str>,
    bytecode: Arc<[u8]>,
    created_at: OffsetDateTime,
    permits: Arc<Semaphore>,
}

struct VmPermit {
    _permit: OwnedSemaphorePermit,
}

struct ExecutionLimits {
    checkers: Arc<Semaphore>,
    other: Arc<Semaphore>,
}

impl ExecutionLimits {
    fn new(checkers: usize, other: usize) -> Self {
        Self {
            checkers: Arc::new(Semaphore::new(checkers)),
            other: Arc::new(Semaphore::new(other)),
        }
    }

    // Fail fast: queued submissions remain in JetStream instead of consuming
    // their processing lease while waiting for a VM.
    fn try_acquire(
        &self,
        key: &str,
        script: Arc<Semaphore>,
    ) -> Result<(OwnedSemaphorePermit, OwnedSemaphorePermit), EngineError> {
        let script = script.try_acquire_owned().map_err(|_| EngineError::Busy)?;
        let domain = if key.starts_with("challenge/") {
            &self.checkers
        } else {
            &self.other
        };
        let domain = domain
            .clone()
            .try_acquire_owned()
            .map_err(|_| EngineError::Busy)?;
        Ok((script, domain))
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
    create_lua_with_deadline(Instant::now() + LUA_CALL_TIMEOUT)
}

fn create_lua_with_deadline(deadline: Instant) -> Result<Lua, EngineError> {
    let permit = VM_PERMITS
        .clone()
        .try_acquire_owned()
        .map_err(|_| EngineError::Busy)?;
    let libs = StdLib::COROUTINE | StdLib::TABLE | StdLib::STRING | StdLib::UTF8 | StdLib::MATH;
    let lua = Lua::new_with(libs, LuaOptions::default())?;
    lua.set_app_data(VmPermit { _permit: permit });
    lua.set_memory_limit(LUA_MEMORY_LIMIT)?;

    let instruction_batches = Arc::new(AtomicU32::new(0));
    lua.set_global_hook(
        HookTriggers::new().every_nth_instruction(LUA_INSTRUCTION_BATCH),
        move |_lua, _debug| {
            if Instant::now() >= deadline {
                return Err(mlua::Error::RuntimeError(
                    "script execution timed out".to_owned(),
                ));
            }
            if instruction_batches.fetch_add(1, Ordering::Relaxed) >= LUA_INSTRUCTION_BATCH_LIMIT {
                Err(mlua::Error::RuntimeError(
                    "script instruction limit exceeded".to_owned(),
                ))
            } else {
                Ok(VmState::Continue)
            }
        },
    )?;

    logging::install(&lua)?;
    modules::install(&lua)?;
    Ok(lua)
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

/// Caches compiled bytecode. Each execution owns a fresh, bounded Lua state.
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
    let bytecode: Arc<[u8]> = Arc::from(function.dump(false));
    let script: Arc<str> = Arc::from(script);
    let mut entry = GLOBAL_ENGINE
        .entry(key.to_owned())
        .or_insert_with(|| EngineContext {
            script: script.clone(),
            bytecode: bytecode.clone(),
            created_at: OffsetDateTime::now_utc(),
            permits: Arc::new(Semaphore::new(MAX_SCRIPT_EXECUTIONS)),
        });
    entry.script = script;
    entry.bytecode = bytecode;
    entry.created_at = OffsetDateTime::now_utc();
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
    let permits = context.permits.clone();
    drop(context);

    let _permits = EXECUTION_LIMITS.try_acquire(key.as_ref(), permits)?;
    let deadline = tokio::time::Instant::now() + LUA_CALL_TIMEOUT;
    let started = Instant::now();
    let lua = create_lua()?;
    configure(&lua)?;
    lua.load(bytecode.as_ref())
        .set_mode(ChunkMode::Binary)
        .exec()?;
    let function = lua
        .globals()
        .get::<Function>(function)
        .map_err(|_| EngineError::MissingFunction(function.to_owned()))?;
    let result: Result<Value, EngineError> =
        tokio::time::timeout_at(deadline, function.call_async(args))
            .await
            .map_err(|_| EngineError::Timeout)?
            .map_err(EngineError::from);
    let value = result?;
    let output = lua.from_value(value)?;
    debug!(
        key = key.as_ref(),
        elapsed_ms = started.elapsed().as_millis(),
        lua_bytes = lua.used_memory(),
        "Lua execution completed"
    );
    Ok(output)
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
    let permits = context.permits.clone();
    drop(context);

    let _permits = EXECUTION_LIMITS.try_acquire(key.as_ref(), permits)?;
    let deadline = tokio::time::Instant::now() + LUA_CALL_TIMEOUT;
    let started = Instant::now();
    let lua = create_lua()?;
    configure(&lua)?;
    lua.load(bytecode.as_ref())
        .set_mode(ChunkMode::Binary)
        .exec()?;
    let function = lua
        .globals()
        .get::<Function>(function)
        .map_err(|_| EngineError::MissingFunction(function.to_owned()))?;
    let mut values = MultiValue::new();
    for arg in args {
        values.push_back(lua.to_value(arg)?);
    }
    let result: Result<Value, EngineError> =
        tokio::time::timeout_at(deadline, function.call_async(values))
            .await
            .map_err(|_| EngineError::Timeout)?
            .map_err(EngineError::from);
    let value = result?;
    let output = lua.from_value(value)?;
    debug!(
        key = key.as_ref(),
        elapsed_ms = started.elapsed().as_millis(),
        lua_bytes = lua.used_memory(),
        "Lua execution completed"
    );
    Ok(output)
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
    use std::{
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::Duration,
    };

    use mlua::Lua;

    use super::{ConfigureLua, clear_cache, execute, lint, preload};
    use crate::traits::EngineError;

    static ENGINE_TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

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
    async fn limits_instructions_in_async_execution_and_recovers() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "test/runtime-instruction-limit";
        preload(
            key,
            r#"
                function spin()
                    while true do end
                end
                function value()
                    return 7
                end
            "#,
            None,
        )
        .await
        .unwrap();

        let error = execute::<_, i64>(key, "spin", (), configure())
            .await
            .unwrap_err();
        assert!(error.to_string().contains("instruction limit"));

        let value: i64 = execute(key, "value", (), configure()).await.unwrap();
        assert_eq!(value, 7);
    }

    #[tokio::test]
    async fn drops_cancelled_lua_states_instead_of_reusing_them() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "test/cancelled-state";
        let created = Arc::new(AtomicUsize::new(0));
        let created_by_configure = Arc::clone(&created);
        let configure = move |_lua: &Lua| {
            created_by_configure.fetch_add(1, Ordering::Relaxed);
            Ok(())
        };
        preload(
            key,
            r#"
                function pause()
                    time.sleep(60)
                    return 0
                end
                function value()
                    return 7
                end
            "#,
            None,
        )
        .await
        .unwrap();

        let cancelled = tokio::time::timeout(
            Duration::from_millis(10),
            execute::<_, i64>(key, "pause", (), &configure),
        )
        .await;
        assert!(cancelled.is_err());

        let value: i64 = execute(key, "value", (), &configure).await.unwrap();
        assert_eq!(value, 7);
        assert_eq!(created.load(Ordering::Relaxed), 2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "candidate extreme validation"]
    async fn candidate_extreme_recovers_after_concurrent_instruction_exhaustion() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "test/extreme-instruction-limit";
        preload(
            key,
            r#"
                function spin()
                    while true do end
                end
                function value()
                    return 7
                end
            "#,
            None,
        )
        .await
        .unwrap();

        let attempts = std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1)
            .min(16)
            * 4;
        let run = async {
            let mut tasks = tokio::task::JoinSet::new();
            for _ in 0..attempts {
                tasks.spawn(async move {
                    execute::<_, i64>(key, "spin", (), configure())
                        .await
                        .unwrap_err()
                        .to_string()
                });
            }
            while let Some(result) = tasks.join_next().await {
                assert!(result.unwrap().contains("instruction limit"));
            }

            for _ in 0..attempts {
                let value: i64 = execute(key, "value", (), configure()).await.unwrap();
                assert_eq!(value, 7);
            }
        };
        tokio::time::timeout(Duration::from_secs(30), run)
            .await
            .expect("concurrent instruction exhaustion did not terminate");
    }

    #[tokio::test]
    #[ignore = "candidate extreme validation"]
    async fn candidate_extreme_discards_repeatedly_cancelled_states() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "test/extreme-cancelled-state";
        let created = Arc::new(AtomicUsize::new(0));
        let created_by_configure = Arc::clone(&created);
        let configure = move |_lua: &Lua| {
            created_by_configure.fetch_add(1, Ordering::Relaxed);
            Ok(())
        };
        preload(
            key,
            r#"
                function pause()
                    time.sleep(60)
                    return 0
                end
                function value()
                    return 7
                end
            "#,
            None,
        )
        .await
        .unwrap();

        for _ in 0..64 {
            let cancelled = tokio::time::timeout(
                Duration::from_millis(5),
                execute::<_, i64>(key, "pause", (), &configure),
            )
            .await;
            assert!(cancelled.is_err());
        }
        assert_eq!(created.load(Ordering::Relaxed), 64);

        for _ in 0..64 {
            let value: i64 = execute(key, "value", (), &configure).await.unwrap();
            assert_eq!(value, 7);
        }
        assert_eq!(created.load(Ordering::Relaxed), 128);
    }

    #[tokio::test]
    #[ignore = "candidate extreme validation"]
    async fn candidate_extreme_recovers_after_repeated_memory_exhaustion() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "test/extreme-memory-limit";
        preload(
            key,
            r#"
                function exhaust()
                    local values = {}
                    while true do
                        values[#values + 1] = string.rep("x", 1024 * 1024)
                    end
                end
                function value()
                    return 7
                end
            "#,
            None,
        )
        .await
        .unwrap();

        let run = async {
            for _ in 0..16 {
                let error = execute::<_, i64>(key, "exhaust", (), configure())
                    .await
                    .unwrap_err();
                assert!(error.to_string().to_ascii_lowercase().contains("memory"));

                let value: i64 = execute(key, "value", (), configure()).await.unwrap();
                assert_eq!(value, 7);
            }
        };
        tokio::time::timeout(Duration::from_secs(30), run)
            .await
            .expect("repeated memory exhaustion did not terminate");
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
        let _guard = ENGINE_TEST_LOCK.lock().await;
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
    async fn isolates_globals_between_executions() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
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
    async fn isolates_string_metatables_and_gc_settings_for_both_entry_points() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "test/vm-state-isolation";
        preload(
            key,
            r#"
            function value()
                local library = getmetatable("").__index
                library.counter = (library.counter or 0) + 1
                local running = collectgarbage("isrunning")
                collectgarbage("stop")
                return {library.counter, running}
            end
        "#,
            None,
        )
        .await
        .unwrap();
        for _ in 0..3 {
            let value: (i64, bool) = execute(key, "value", (), configure()).await.unwrap();
            assert_eq!(value, (1, true));
            let value: (i64, bool) = super::execute_json(key, "value", &[], configure())
                .await
                .unwrap();
            assert_eq!(value, (1, true));
        }
    }

    #[test]
    fn enforces_wall_clock_deadline_in_synchronous_lua() {
        let lua = super::create_lua_with_deadline(std::time::Instant::now()).unwrap();
        let error = lua.load("while true do end").exec().unwrap_err();
        assert!(error.to_string().contains("timed out"));
    }

    #[test]
    fn limits_scripts_and_domains_without_leaking_partial_reservations() {
        use tokio::sync::Semaphore;
        let limits = super::ExecutionLimits::new(2, 1);
        let hot = Arc::new(Semaphore::new(1));
        let cold = Arc::new(Semaphore::new(1));
        let first = limits.try_acquire("challenge/hot", hot.clone()).unwrap();
        assert!(matches!(
            limits.try_acquire("challenge/hot", hot.clone()),
            Err(EngineError::Busy)
        ));
        let second = limits.try_acquire("challenge/cold", cold.clone()).unwrap();
        let third = Arc::new(Semaphore::new(1));
        assert!(matches!(
            limits.try_acquire("challenge/third", third.clone()),
            Err(EngineError::Busy)
        ));
        assert_eq!(third.available_permits(), 1);
        let idp = limits
            .try_acquire("idp/login", Arc::new(Semaphore::new(1)))
            .unwrap();
        assert!(matches!(
            limits.try_acquire("idp/other", Arc::new(Semaphore::new(1))),
            Err(EngineError::Busy)
        ));
        drop((first, second, idp));
        assert!(limits.try_acquire("challenge/third", third).is_ok());
    }

    #[tokio::test]
    async fn rejects_saturation_and_recovers_after_cancellation_and_reload() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
        let key = "challenge/test-admission";
        let script = "function pause() time.sleep(60) return 0 end function value() return 7 end";
        preload(key, script, None).await.unwrap();
        let created = Arc::new(AtomicUsize::new(0));
        let configure = Arc::new({
            let created = created.clone();
            move |_: &Lua| {
                created.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        });
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..super::MAX_SCRIPT_EXECUTIONS {
            let configure = configure.clone();
            tasks.spawn(
                async move { execute::<_, i64>(key, "pause", (), configure.as_ref()).await },
            );
        }
        tokio::time::timeout(Duration::from_secs(3), async {
            while created.load(Ordering::Relaxed) < super::MAX_SCRIPT_EXECUTIONS {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        // Source updates retain the old version's quota until its calls finish.
        preload(key, format!("{script} -- new version"), None)
            .await
            .unwrap();
        assert!(matches!(
            execute::<_, i64>(key, "value", (), configure.as_ref()).await,
            Err(EngineError::Busy)
        ));
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
        let value: i64 = execute(key, "value", (), configure.as_ref()).await.unwrap();
        assert_eq!(value, 7);
    }

    #[tokio::test]
    async fn exposes_time_module_during_execution() {
        let _guard = ENGINE_TEST_LOCK.lock().await;
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

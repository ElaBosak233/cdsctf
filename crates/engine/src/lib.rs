//! Embedded [Rune](https://rune-rs.github.io/) script engine used for challenge checkers and tooling.
//!
//! Compiled scripts are cached in a process-wide [`DashMap`] under string keys.
//! [`preload`] skips work when the cached unit is newer than the script’s
//! `updated_at` timestamp from the database.

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Defines the `util` submodule (see sibling `*.rs` files).
mod util;

/// Defines the `worker` submodule (see sibling `*.rs` files).
mod worker;

use std::{collections::HashSet, sync::Arc};

use dashmap::DashMap;
use once_cell::sync::Lazy;
pub use rune;
use rune::{
    Context, Diagnostics, Module, Source, Sources, Unit, Value, Vm,
    runtime::{Args, RuntimeContext},
};
pub use rune_modules;
use time::OffsetDateTime;
use tracing::debug;

use crate::traits::{DiagnosticKind, DiagnosticMarker, EngineError};

/// One compiled script: immutable [`Unit`], shared runtime, and cache insertion
/// time for staleness checks.
struct EngineContext {
    pub unit: Arc<Unit>,
    pub runtime_context: Arc<RuntimeContext>,
    pub created_at: OffsetDateTime,
}

/// Global script cache keyed by logical names such as `challenge/{id}`.
static GLOBAL_ENGINE: Lazy<Arc<DashMap<String, EngineContext>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

/// Returns global engine.

fn get_global_engine() -> Arc<DashMap<String, EngineContext>> {
    Arc::clone(&GLOBAL_ENGINE)
}

/// Starts maintenance tasks (e.g. evicting stale engine entries) — currently
/// delegates to `worker::cleaner`.
pub async fn init() -> Result<(), EngineError> {
    worker::cleaner().await;

    Ok(())
}

/// Builds a fresh Rune [`Context`] with default modules plus caller-supplied
/// native modules.
pub async fn gen_rune_context<M>(modules: Vec<M>) -> Result<Context, EngineError>
where
    M: AsRef<Module>, {
    let mut context = Context::with_default_modules()?;
    for module in modules {
        context.install(module)?;
    }
    Ok(context)
}

/// Compiles `script` for diagnostics, collects structured markers, and verifies
/// `required_functions` exist on the VM.
pub async fn lint(
    context: Context,
    script: impl AsRef<str>,
    required_functions: &[&'static str],
) -> Result<(), EngineError> {
    let mut sources = Sources::new();
    sources.insert(Source::memory(script)?)?;

    let mut diagnostics = Diagnostics::new();
    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    let mut markers_set: HashSet<String> = HashSet::new();
    let mut markers: Vec<DiagnosticMarker> = Vec::new();

    for diagnostic in diagnostics.diagnostics() {
        if let Some(marker) = util::diagnostic_to_marker(diagnostic, &sources) {
            let key = format!(
                "{:?}:{:?}:{:?}",
                marker.kind, marker.message, marker.start_line
            );
            if markers_set.insert(key) {
                markers.push(marker);
            }
        }
    }

    let unit = match result {
        Ok(unit) => unit,
        Err(error) => {
            if markers.is_empty() {
                markers.push(DiagnosticMarker {
                    kind: DiagnosticKind::Error,
                    message: format!("Script failed to compile: {}", error),
                    start_line: 0,
                    start_column: 0,
                    end_line: 0,
                    end_column: 0,
                });
            }
            return Err(EngineError::DiagnosticsError(markers));
        }
    };

    let runtime = context.runtime()?;
    let vm = Vm::new(Arc::new(runtime), Arc::new(unit));

    for func in required_functions {
        if vm.lookup_function([func]).is_err() {
            let msg = format!("Missing required function: {}", func);
            if markers_set.insert(msg.clone()) {
                markers.push(DiagnosticMarker {
                    kind: DiagnosticKind::Error,
                    message: msg,
                    start_line: 0,
                    start_column: 0,
                    end_line: 0,
                    end_column: 0,
                });
            }
        }
    }

    if !markers.is_empty() {
        return Err(EngineError::DiagnosticsError(markers));
    }

    Ok(())
}

/// Stores a compiled `script` under `key`, replacing it only if missing or
/// older than `last_changed_at`.
pub async fn preload(
    context: Context,
    key: impl AsRef<str>,
    script: impl AsRef<str>,
    last_changed_at: Option<OffsetDateTime>,
) -> Result<(), EngineError> {
    let global_engine = get_global_engine();

    if let Some(context) = global_engine.get(key.as_ref()) {
        if let Some(last_changed_at) = last_changed_at {
            if context.created_at.gt(&last_changed_at) {
                debug!(key = key.as_ref(), "Engine is up to date, skipping");
                return Ok(());
            }
        }
    }

    debug!(key = key.as_ref(), "Preloading engine");

    let mut sources = Sources::new();

    sources.insert(Source::memory(&script)?)?;

    let unit = rune::prepare(&mut sources).with_context(&context).build()?;
    let runtime = context.runtime()?;

    global_engine.insert(
        key.as_ref().to_string(),
        EngineContext {
            unit: Arc::new(unit),
            runtime_context: Arc::new(runtime),
            created_at: OffsetDateTime::now_utc(),
        },
    );

    Ok(())
}

/// Runs `function` on the cached script for `key` with the given Rune
/// arguments.
pub async fn execute(
    key: impl AsRef<str>,
    function: &'static str,
    args: impl Args + Send,
) -> Result<Value, EngineError> {
    let global_engine = get_global_engine();
    let context = global_engine
        .get(key.as_ref())
        .ok_or(EngineError::MissingContext(format!("{}", key.as_ref())))?;

    let vm = Vm::new(context.runtime_context.clone(), context.unit.clone());
    let result = vm.send_execute([function], args)?;
    let result = result.async_complete().await.into_result()?;

    Ok(result)
}

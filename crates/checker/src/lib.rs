pub mod modules;
pub mod traits;
pub mod util;
pub mod worker;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use rune::{
    Context, Diagnostics, Source, Sources, Unit, Value, Vm,
    runtime::{Object, RuntimeContext},
};
use time::OffsetDateTime;
use tracing::{debug, warn};

pub use crate::modules::audit::Status;
use crate::traits::{CheckerError, DiagnosticKind, DiagnosticMarker};

struct CheckerContext {
    pub unit: Arc<Unit>,
    pub runtime_context: Arc<RuntimeContext>,
    pub created_at: OffsetDateTime,
}

static CHECKER_CONTEXT: Lazy<Arc<DashMap<i64, CheckerContext>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

pub async fn init() -> Result<(), CheckerError> {
    worker::cleaner().await;

    Ok(())
}

async fn gen_rune_context(challenge_id: i64) -> Result<Context, CheckerError> {
    let mut rune_context = Context::with_default_modules()?;
    rune_context.install(rune_modules::http::module(true)?)?;
    rune_context.install(rune_modules::json::module(true)?)?;
    rune_context.install(rune_modules::toml::module(true)?)?;
    rune_context.install(rune_modules::process::module(true)?)?;

    rune_context.install(modules::audit::module(true)?)?;
    rune_context.install(modules::crypto::module(true)?)?;
    rune_context.install(modules::regex::module(true)?)?;
    rune_context.install(modules::suid::module(true)?)?;
    rune_context.install(modules::leet::module(true)?)?;

    rune_context.install(modules::fs::module(true, challenge_id).await?)?;

    Ok(rune_context)
}

fn get_checker_context() -> Arc<DashMap<i64, CheckerContext>> {
    Arc::clone(&CHECKER_CONTEXT)
}

pub async fn lint(challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
    let context = gen_rune_context(challenge.id).await?;
    let mut sources = Sources::new();
    let script = challenge
        .clone()
        .checker
        .ok_or_else(|| CheckerError::MissingScript("".to_owned()))?;
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
            return Err(CheckerError::DiagnosticsError(markers));
        }
    };

    let runtime = context.runtime()?;
    let vm = Vm::new(Arc::new(runtime), Arc::new(unit));

    for func in ["check", "generate"] {
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
        return Err(CheckerError::DiagnosticsError(markers));
    }

    Ok(())
}

async fn preload(challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
    let rune_context = gen_rune_context(challenge.id).await?;
    let checker_context = get_checker_context();

    if let Some(context) = checker_context.get(&challenge.id) {
        if context.created_at.unix_timestamp() > challenge.updated_at {
            return Ok(());
        }
    }

    debug!("Preloading checker for challenge {}", challenge.id);

    let mut sources = Sources::new();

    let script = challenge
        .clone()
        .checker
        .ok_or(CheckerError::MissingScript("".to_owned()))?;

    sources.insert(Source::memory(&script)?)?;
    lint(challenge).await?;

    let unit = rune::prepare(&mut sources)
        .with_context(&rune_context)
        .build()?;
    let runtime = rune_context.runtime()?;

    checker_context.insert(
        challenge.id,
        CheckerContext {
            unit: Arc::new(unit),
            runtime_context: Arc::new(runtime),
            created_at: OffsetDateTime::now_utc(),
        },
    );

    Ok(())
}

pub async fn check(
    challenge: &cds_db::Challenge,
    operator_id: i64,
    content: &str,
) -> Result<Status, CheckerError> {
    preload(challenge).await?;

    let checker_context = get_checker_context();
    let ctx = checker_context
        .get(&challenge.id)
        .ok_or(CheckerError::MissingScript("".to_owned()))?;
    let vm = Vm::new(ctx.runtime_context.clone(), ctx.unit.clone());

    let result = vm
        .send_execute(["check"], (operator_id, content))?
        .async_complete()
        .await
        .into_result()?;
    let output = rune::from_value::<Result<Status, Value>>(result)?;

    let is_correct = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

    Ok(is_correct)
}

pub async fn generate(
    challenge: &cds_db::Challenge,
    operator_id: i64,
) -> Result<HashMap<String, String>, CheckerError> {
    preload(challenge).await?;

    let checker_context = get_checker_context();
    let ctx = checker_context
        .get(&challenge.id)
        .ok_or(CheckerError::MissingScript("".to_owned()))?;
    let vm = Vm::new(ctx.runtime_context.clone(), ctx.unit.clone());

    let result = vm
        .send_execute(["generate"], (operator_id,))?
        .async_complete()
        .await
        .into_result()?;
    let output = rune::from_value::<Result<Object, Value>>(result)?;

    let object = output.map_err(|err| CheckerError::ScriptError(format!("{:?}", err)))?;

    let mut environs = HashMap::new();
    for (key, value) in object.iter() {
        environs.insert(key.to_string(), rune::from_value(value.to_owned())?);
    }

    Ok(environs)
}

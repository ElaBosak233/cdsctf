pub mod modules;
pub mod traits;
pub mod util;
pub mod worker;

use std::{collections::HashMap, ops::Deref, sync::Arc};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::{Lazy, OnceCell};
use rune::{
    Any, Context, Diagnostics, Source, Sources, Unit, Value, Vm,
    runtime::{Object, RuntimeContext},
    termcolor::Buffer,
};
use tracing::{debug, info};
use uuid::Uuid;

pub use crate::modules::audit::Status;
use crate::traits::CheckerError;

struct CheckerContext {
    pub unit: Arc<Unit>,
    pub runtime_context: Arc<RuntimeContext>,
    pub created_at: DateTime<Utc>,
}

static CHECKER_CONTEXT: Lazy<Arc<DashMap<Uuid, CheckerContext>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

static RUNE_CONTEXT: OnceCell<Context> = OnceCell::new();

pub async fn init() -> Result<(), CheckerError> {
    fn init_rune_context() -> Result<(), CheckerError> {
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

        RUNE_CONTEXT
            .set(rune_context)
            .map_err(|_| anyhow!("Failed to set rune_context into OnceCell."))?;

        info!("Checker's rune context loaded.");

        Ok(())
    }

    init_rune_context()?;
    worker::cleaner().await;

    Ok(())
}

fn get_checker_context() -> Arc<DashMap<Uuid, CheckerContext>> {
    Arc::clone(&CHECKER_CONTEXT)
}

fn get_rune_context() -> &'static Context {
    &RUNE_CONTEXT.get().unwrap()
}

pub fn lint(script: &str) -> Result<(), CheckerError> {
    let mut sources = Sources::new();
    sources.insert(Source::memory(script)?)?;
    let mut diagnostics = Diagnostics::new();

    let _ = rune::prepare(&mut sources)
        .with_context(&get_rune_context())
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut out = Buffer::ansi();
        diagnostics.emit(&mut out, &sources)?;

        let out = String::from_utf8(out.into_inner())?.to_string();

        return Err(CheckerError::CompileError(out));
    }

    let unit = rune::prepare(&mut sources)
        .with_context(&get_rune_context())
        .build()?;

    let runtime = get_rune_context().runtime()?;
    let vm = Vm::new(Arc::new(runtime), Arc::new(unit));

    vm.lookup_function(["check"])
        .map_err(|_| CheckerError::MissingFunction("check".to_owned()))?;

    vm.lookup_function(["environ"])
        .map_err(|_| CheckerError::MissingFunction("environ".to_owned()))?;

    Ok(())
}

async fn preload(challenge: &cds_db::transfer::Challenge) -> Result<(), CheckerError> {
    let checker_context = get_checker_context();

    if let Some(context) = checker_context.get(&challenge.id) {
        if context.created_at.timestamp() > challenge.updated_at {
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
    lint(&script)?;

    let unit = rune::prepare(&mut sources)
        .with_context(&get_rune_context())
        .build()?;
    let runtime = get_rune_context().runtime()?;

    checker_context.insert(challenge.id, CheckerContext {
        unit: Arc::new(unit),
        runtime_context: Arc::new(runtime),
        created_at: Utc::now(),
    });

    Ok(())
}

pub async fn check(
    challenge: &cds_db::transfer::Challenge, operator_id: i64, content: &str,
) -> Result<Status, CheckerError> {
    preload(&challenge).await?;

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

pub async fn environ(
    challenge: &cds_db::transfer::Challenge, operator_id: i64,
) -> Result<HashMap<String, String>, CheckerError> {
    preload(&challenge).await?;

    let checker_context = get_checker_context();
    let ctx = checker_context
        .get(&challenge.id)
        .ok_or(CheckerError::MissingScript("".to_owned()))?;
    let vm = Vm::new(ctx.runtime_context.clone(), ctx.unit.clone());

    let result = vm
        .send_execute(["environ"], (operator_id,))?
        .async_complete()
        .await
        .into_result()?;
    let output = rune::from_value::<Result<Object, Value>>(result)?;

    let object = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

    let mut environs = HashMap::new();
    for (key, value) in object.iter() {
        environs.insert(key.to_string(), rune::from_value(value.to_owned())?);
    }

    Ok(environs)
}

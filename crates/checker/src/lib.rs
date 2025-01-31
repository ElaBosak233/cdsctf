pub mod modules;
pub mod traits;
pub mod worker;

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::{Lazy, OnceCell};
use rune::{Context, Diagnostics, Source, Sources, Unit, Vm, runtime::RuntimeContext, termcolor::Buffer, Any, Value};
use rune::runtime::Object;
use uuid::Uuid;

use crate::traits::CheckerError;

pub struct CheckerContext {
    pub unit: Arc<Unit>,
    pub runtime_context: Arc<RuntimeContext>,
    pub created_at: DateTime<Utc>,
}

pub static CHECKER_CONTEXT: Lazy<Arc<DashMap<Uuid, CheckerContext>>> =
    Lazy::new(|| Arc::new(DashMap::new()));

pub static RUNE_CONTEXT: OnceCell<Context> = OnceCell::new();

pub async fn init() -> Result<(), CheckerError> {
    init_rune_context()?;
    worker::cleaner().await;

    Ok(())
}

pub fn get_checker_context() -> Arc<DashMap<Uuid, CheckerContext>> {
    Arc::clone(&CHECKER_CONTEXT)
}

pub fn init_rune_context() -> Result<(), CheckerError> {
    let mut rune_context = Context::with_default_modules()?;
    rune_context.install(rune_modules::http::module(true)?)?;
    rune_context.install(rune_modules::json::module(true)?)?;
    rune_context.install(rune_modules::toml::module(true)?)?;
    rune_context.install(rune_modules::process::module(true)?)?;

    rune_context.install(modules::crypto::module(true)?)?;

    RUNE_CONTEXT.set(rune_context).map_err(|_| {
        CheckerError::OtherError(anyhow!("RUNE_CONTEXT has already been initialized"))
    })?;

    Ok(())
}

pub fn get_rune_context() -> &'static Context {
    RUNE_CONTEXT.get().unwrap()
}

pub async fn lint(script: &str) -> Result<(), CheckerError> {
    let rune_context = get_rune_context();
    let mut sources = Sources::new();
    sources.insert(Source::memory(script)?)?;
    let mut diagnostics = Diagnostics::new();

    let _ = rune::prepare(&mut sources)
        .with_context(&rune_context)
        .with_diagnostics(&mut diagnostics);

    if !diagnostics.is_empty() {
        let mut out = Buffer::ansi();
        diagnostics.emit(&mut out, &sources)?;

        return Err(CheckerError::CompileError(
            String::from_utf8_lossy(&out.into_inner()).to_string(),
        ));
    }

    let unit = rune::prepare(&mut sources)
        .with_context(&rune_context)
        .build()?;
    let runtime = rune_context.runtime()?;
    let vm = Vm::new(Arc::new(runtime), Arc::new(unit));

    vm.lookup_function(["check"])
        .map_err(|_| CheckerError::MissingFunction("check".to_owned()))?;

    vm.lookup_function(["environ"])
        .map_err(|_| CheckerError::MissingFunction("environ".to_owned()))?;

    Ok(())
}

async fn preload(challenge: &cds_db::transfer::Challenge) -> Result<(), CheckerError> {
    let checker_context = get_checker_context();

    if checker_context.contains_key(&challenge.id) {
        return Ok(());
    }

    let mut sources = Sources::new();

    if let Some(script) = &challenge.script {
        sources.insert(Source::memory(script)?)?;
    } else {
        return Err(CheckerError::MissingScript("".to_owned()));
    }

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

pub async fn check(challenge: cds_db::transfer::Challenge, operator_id: i64, content: &str) -> Result<bool, CheckerError> {
    preload(&challenge).await?;

    let checker_context = get_checker_context();
    let ctx = checker_context.get(&challenge.id).ok_or(CheckerError::MissingScript("".to_owned()))?;
    let vm = Vm::new(ctx.runtime_context.clone(), ctx.unit.clone());

    let result = vm.send_execute(["check"], (operator_id, content))?.async_complete().await.into_result()?;
    let output = rune::from_value::<Result<bool, Value>>(result)?;

    let is_correct = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

    Ok(is_correct)
}

pub async fn environ(challenge: cds_db::transfer::Challenge, operator_id: i64) -> Result<HashMap<String, String>, CheckerError> {
    preload(&challenge).await?;

    let checker_context = get_checker_context();
    let ctx = checker_context.get(&challenge.id).ok_or(CheckerError::MissingScript("".to_owned()))?;
    let vm = Vm::new(ctx.runtime_context.clone(), ctx.unit.clone());

    let result = vm.send_execute(["environ"], (operator_id, ))?.async_complete().await.into_result()?;
    let output = rune::from_value::<Result<Object, Value>>(result)?;

    let object = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

    let mut environs = HashMap::new();
    for (key, value) in object.iter() {
        environs.insert(key.to_string(), rune::from_value(value.to_owned())?);
    }

    Ok(environs)
}
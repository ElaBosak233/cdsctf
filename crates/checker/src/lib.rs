pub mod modules;
pub mod traits;
pub mod util;
pub mod worker;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::{Lazy, OnceCell};
use rune::{
    Context, Diagnostics, Source, Sources, Unit, Value, Vm,
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

pub async fn init() -> Result<(), CheckerError> {
    worker::cleaner().await;

    Ok(())
}

async fn gen_rune_context(challenge_id: &Uuid) -> Result<Context, CheckerError> {
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

    rune_context.install(modules::fs::module(
        true,
        cds_media::challenge::get_root_path(challenge_id).await?,
    )?)?;

    Ok(rune_context)
}

fn get_checker_context() -> Arc<DashMap<Uuid, CheckerContext>> {
    Arc::clone(&CHECKER_CONTEXT)
}

pub async fn lint(challenge: &cds_db::transfer::Challenge) -> Result<(), CheckerError> {
    let context = gen_rune_context(&challenge.id).await?;
    let mut sources = Sources::new();
    let script = challenge
        .clone()
        .checker
        .ok_or(CheckerError::MissingScript("".to_owned()))?;
    sources.insert(Source::memory(script)?)?;
    let mut diagnostics = Diagnostics::new();

    let _ = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut out = Buffer::ansi();
        diagnostics.emit(&mut out, &sources)?;

        let out = String::from_utf8(out.into_inner())?.to_string();

        return Err(CheckerError::CompileError(out));
    }

    let unit = rune::prepare(&mut sources).with_context(&context).build()?;

    let runtime = context.runtime()?;
    let vm = Vm::new(Arc::new(runtime), Arc::new(unit));

    vm.lookup_function(["check"])
        .map_err(|_| CheckerError::MissingFunction("check".to_owned()))?;

    vm.lookup_function(["generate"])
        .map_err(|_| CheckerError::MissingFunction("generate".to_owned()))?;

    Ok(())
}

async fn preload(challenge: &cds_db::transfer::Challenge) -> Result<(), CheckerError> {
    let rune_context = gen_rune_context(&challenge.id).await?;
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
    lint(&challenge).await?;

    let unit = rune::prepare(&mut sources)
        .with_context(&rune_context)
        .build()?;
    let runtime = rune_context.runtime()?;

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
    challenge: &cds_db::transfer::Challenge, operator_id: i64,
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

    let object = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

    let mut environs = HashMap::new();
    for (key, value) in object.iter() {
        environs.insert(key.to_string(), rune::from_value(value.to_owned())?);
    }

    Ok(environs)
}

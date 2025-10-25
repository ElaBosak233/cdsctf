pub mod modules;
pub mod traits;
pub mod util;

use std::collections::HashMap;

use cds_engine::{
    rune::{Context, Value, runtime::Object},
    rune_modules,
};
use time::OffsetDateTime;
use tracing::debug;

pub use crate::modules::audit::Status;
use crate::traits::CheckerError;

async fn gen_rune_context(challenge_id: i64) -> Result<Context, CheckerError> {
    Ok(cds_engine::gen_rune_context(vec![
        rune_modules::http::module(true)?,
        rune_modules::json::module(true)?,
        rune_modules::toml::module(true)?,
        rune_modules::process::module(true)?,
        modules::audit::module(true)?,
        modules::crypto::module(true)?,
        modules::regex::module(true)?,
        modules::suid::module(true)?,
        modules::leet::module(true)?,
        modules::fs::module(true, challenge_id).await?,
    ])
    .await?)
}

pub async fn lint(challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
    cds_engine::lint(
        gen_rune_context(challenge.id).await?,
        challenge
            .checker
            .clone()
            .ok_or(CheckerError::MissingScript("".to_owned()))?,
        &["check", "generate"],
    )
    .await?;

    Ok(())
}

async fn preload(challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
    cds_engine::preload(
        gen_rune_context(challenge.id).await?,
        format!("challenge/{}", challenge.id),
        challenge
            .checker
            .clone()
            .ok_or(CheckerError::MissingScript("".to_owned()))?,
        Some(
            OffsetDateTime::from_unix_timestamp(challenge.created_at)
                .ok()
                .unwrap_or(OffsetDateTime::now_utc()),
        ),
    )
    .await?;

    Ok(())
}

pub async fn check(
    challenge: &cds_db::Challenge,
    operator_id: i64,
    content: &str,
) -> Result<Status, CheckerError> {
    preload(challenge).await?;
    debug!(
        challenge_id = challenge.id,
        operator_id = operator_id,
        content = content,
        "Checking answers"
    );
    let result = cds_engine::execute(
        format!("challenge/{}", challenge.id),
        "check",
        (operator_id, content),
    )
    .await?;
    let output = cds_engine::rune::from_value::<Result<Status, Value>>(result)?;

    let is_correct = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

    Ok(is_correct)
}

pub async fn generate(
    challenge: &cds_db::Challenge,
    operator_id: i64,
) -> Result<HashMap<String, String>, CheckerError> {
    preload(challenge).await?;
    debug!(
        challenge_id = challenge.id,
        operator_id = operator_id,
        "Generating environment variables"
    );
    let result = cds_engine::execute(
        format!("challenge/{}", challenge.id),
        "generate",
        (operator_id,),
    )
    .await?;
    let output = cds_engine::rune::from_value::<Result<Object, Value>>(result)?;

    let object = output.map_err(|err| CheckerError::ScriptError(format!("{:?}", err)))?;

    let mut environs = HashMap::new();
    for (key, value) in object.iter() {
        environs.insert(
            key.to_string(),
            cds_engine::rune::from_value(value.to_owned())?,
        );
    }

    Ok(environs)
}

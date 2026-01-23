pub mod modules;
pub mod traits;
pub mod util;

use std::collections::HashMap;

use cds_engine::{
    rune::{Context, Value, runtime::Object},
    rune_modules,
    traits::EngineError,
};
use cds_media::Media;
use time::OffsetDateTime;
use tracing::debug;

pub use crate::modules::audit::Status;
use crate::traits::CheckerError;

#[derive(Clone)]
pub struct Checker {
    media: Media,
}

pub fn init(media: &Media) -> Result<Checker, CheckerError> {
    Ok(Checker {
        media: media.clone(),
    })
}

impl Checker {
    async fn gen_rune_context(&self, challenge_id: i64) -> Result<Context, CheckerError> {
        Ok(cds_engine::gen_rune_context(vec![
            rune_modules::http::module(true).map_err(EngineError::from)?,
            rune_modules::json::module(true).map_err(EngineError::from)?,
            rune_modules::toml::module(true).map_err(EngineError::from)?,
            rune_modules::process::module(true).map_err(EngineError::from)?,
            modules::audit::module(true).map_err(EngineError::from)?,
            modules::crypto::module(true).map_err(EngineError::from)?,
            modules::regex::module(true).map_err(EngineError::from)?,
            modules::suid::module(true).map_err(EngineError::from)?,
            modules::leet::module(true).map_err(EngineError::from)?,
            modules::fs::module(true, self.media.clone(), challenge_id)
                .await
                .map_err(EngineError::from)?,
        ])
        .await?)
    }

    pub async fn lint(&self, challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
        cds_engine::lint(
            self.gen_rune_context(challenge.id).await?,
            challenge
                .checker
                .clone()
                .ok_or(CheckerError::MissingScript("".to_owned()))?,
            &["check", "generate"],
        )
        .await?;

        Ok(())
    }

    async fn preload(&self, challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
        cds_engine::preload(
            self.gen_rune_context(challenge.id).await?,
            format!("challenge/{}", challenge.id),
            challenge
                .checker
                .clone()
                .ok_or(CheckerError::MissingScript("".to_owned()))?,
            Some(
                OffsetDateTime::from_unix_timestamp(challenge.updated_at)
                    .ok()
                    .unwrap_or(OffsetDateTime::now_utc()),
            ),
        )
        .await?;

        Ok(())
    }

    pub async fn check(
        &self,
        challenge: &cds_db::Challenge,
        operator_id: i64,
        content: &str,
    ) -> Result<Status, CheckerError> {
        self.preload(challenge).await?;
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
        let output = cds_engine::rune::from_value::<Result<Status, Value>>(result)
            .map_err(EngineError::from)?;

        let is_correct = output.map_err(|_| CheckerError::ScriptError("".to_owned()))?;

        Ok(is_correct)
    }

    pub async fn generate(
        &self,
        challenge: &cds_db::Challenge,
        operator_id: i64,
    ) -> Result<HashMap<String, String>, CheckerError> {
        self.preload(challenge).await?;
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
        let output = cds_engine::rune::from_value::<Result<Object, Value>>(result)
            .map_err(EngineError::from)?;

        let object = output.map_err(|err| CheckerError::ScriptError(format!("{:?}", err)))?;

        let mut environs = HashMap::new();
        for (key, value) in object.iter() {
            environs.insert(
                key.to_string(),
                cds_engine::rune::from_value(value.to_owned()).map_err(EngineError::from)?,
            );
        }

        Ok(environs)
    }
}

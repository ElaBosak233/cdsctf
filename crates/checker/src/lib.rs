//! Challenge checker powered by the embedded Lua engine.
//!
//! Scripts expose top-level `check` and `generate` functions. Host APIs are
//! available under the `cds` global namespace.

pub mod modules;
pub mod traits;
pub mod util;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use cds_engine::{ConfigureLua, mlua::Lua};
use cds_media::Media;
use serde::Deserialize;
use time::OffsetDateTime;
use tracing::debug;

pub use crate::modules::audit::Status;
use crate::traits::CheckerError;

#[derive(Clone)]
pub struct Checker {
    media: Media,
    key_cache: modules::fs::KeyCache,
}

pub fn init(media: &Media) -> Result<Checker, CheckerError> {
    Ok(Checker {
        media: media.clone(),
        key_cache: Arc::new(RwLock::new(HashMap::new())),
    })
}

impl Checker {
    fn configure_lua(&self, challenge_id: i64) -> Arc<ConfigureLua> {
        let media = self.media.clone();
        let key_cache = self.key_cache.clone();
        Arc::new(move |lua: &Lua| {
            modules::audit::install(lua)?;
            modules::crypto::install(lua)?;
            modules::regex::install(lua)?;
            modules::suid::install(lua)?;
            modules::leet::install(lua)?;
            modules::fs::install(lua, media.clone(), key_cache.clone(), challenge_id)?;
            Ok(())
        })
    }

    pub async fn lint(&self, challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
        let script = challenge
            .checker
            .as_deref()
            .ok_or_else(|| CheckerError::MissingScript(String::new()))?;
        let configure = self.configure_lua(challenge.id);
        cds_engine::lint(script, &["check", "generate"], configure.as_ref()).await?;
        Ok(())
    }

    async fn preload(&self, challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
        cds_engine::preload(
            format!("challenge/{}", challenge.id),
            challenge
                .checker
                .as_deref()
                .ok_or_else(|| CheckerError::MissingScript(String::new()))?,
            Some(
                OffsetDateTime::from_unix_timestamp(challenge.updated_at)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc()),
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
            operator_id, "Checking answer with Lua"
        );
        let configure = self.configure_lua(challenge.id);
        let result: StatusOutput = cds_engine::execute(
            format!("challenge/{}", challenge.id),
            "check",
            (operator_id, content),
            configure.as_ref(),
        )
        .await?;
        result.try_into()
    }

    pub async fn generate(
        &self,
        challenge: &cds_db::Challenge,
        operator_id: i64,
    ) -> Result<HashMap<String, String>, CheckerError> {
        self.preload(challenge).await?;
        debug!(
            challenge_id = challenge.id,
            operator_id, "Generating environment variables"
        );
        let configure = self.configure_lua(challenge.id);
        Ok(cds_engine::execute(
            format!("challenge/{}", challenge.id),
            "generate",
            (operator_id,),
            configure.as_ref(),
        )
        .await?)
    }
}

#[derive(Deserialize)]
struct StatusOutput {
    kind: String,
    operator_id: Option<i64>,
}

impl TryFrom<StatusOutput> for Status {
    type Error = CheckerError;

    fn try_from(output: StatusOutput) -> Result<Self, Self::Error> {
        match output.kind.as_str() {
            "correct" => Ok(Status::Correct),
            "incorrect" => Ok(Status::Incorrect),
            "cheat" => output.operator_id.map(Status::Cheat).ok_or_else(|| {
                CheckerError::ScriptError("cheat status requires operator_id".to_owned())
            }),
            _ => Err(CheckerError::ScriptError(format!(
                "unknown checker status: {}",
                output.kind
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use cds_engine::{ConfigureLua, mlua::Lua};

    use super::{Status, StatusOutput, modules};

    const SIMPLE: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/simple.lua"
    );
    const SUID: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/suid.lua"
    );
    const LEET: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/leet.lua"
    );

    fn configure() -> Arc<ConfigureLua> {
        Arc::new(|lua: &Lua| {
            modules::audit::install(lua)?;
            modules::crypto::install(lua)?;
            modules::regex::install(lua)?;
            modules::suid::install(lua)?;
            modules::leet::install(lua)?;
            let fs = cds_engine::module(lua, "fs")?;
            fs.set("key", lua.create_function(|_, ()| Ok("11".repeat(64)))?)?;
            Ok(())
        })
    }

    #[test]
    fn decodes_checker_status() {
        let status = StatusOutput {
            kind: "cheat".to_owned(),
            operator_id: Some(42),
        };
        assert_eq!(Status::try_from(status).unwrap(), Status::Cheat(42));
    }

    #[tokio::test]
    async fn bundled_templates_lint_and_execute() {
        let configure = configure();
        for script in [SIMPLE, SUID, LEET] {
            cds_engine::lint(script, &["check", "generate"], configure.as_ref())
                .await
                .unwrap();
        }

        cds_engine::preload("test/simple", SIMPLE, None)
            .await
            .unwrap();
        let status: StatusOutput = cds_engine::execute(
            "test/simple",
            "check",
            (1_i64, "flag{this_is_my_flag}"),
            configure.as_ref(),
        )
        .await
        .unwrap();
        assert_eq!(Status::try_from(status).unwrap(), Status::Correct);

        for (key, script) in [("test/suid", SUID), ("test/leet", LEET)] {
            cds_engine::preload(key, script, None).await.unwrap();
            let mut generated: HashMap<String, String> =
                cds_engine::execute(key, "generate", (7_i64,), configure.as_ref())
                    .await
                    .unwrap();
            let flag = generated.remove("FLAG").unwrap();

            let correct: StatusOutput =
                cds_engine::execute(key, "check", (7_i64, flag.as_str()), configure.as_ref())
                    .await
                    .unwrap();
            assert_eq!(Status::try_from(correct).unwrap(), Status::Correct);

            let cheat: StatusOutput =
                cds_engine::execute(key, "check", (8_i64, flag.as_str()), configure.as_ref())
                    .await
                    .unwrap();
            assert_eq!(Status::try_from(cheat).unwrap(), Status::Cheat(7));
        }
    }
}

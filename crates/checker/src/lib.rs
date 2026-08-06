//! Challenge checker powered by the embedded Lua engine.
//!
//! Scripts expose top-level `check` and `generate` functions. Checker-specific
//! APIs are available under the `checker` global namespace.

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
    fn configure_lua(&self, challenge_id: i64, default_key: Option<String>) -> Arc<ConfigureLua> {
        let media = self.media.clone();
        Arc::new(move |lua: &Lua| {
            modules::audit::install(lua)?;
            modules::suid::install(lua, default_key.clone())?;
            modules::leet::install(lua, default_key.clone())?;
            modules::fs::install(lua, media.clone(), challenge_id)?;
            Ok(())
        })
    }

    async fn default_key(&self, challenge_id: i64) -> Result<String, CheckerError> {
        if let Some(key) = self
            .key_cache
            .read()
            .map_err(|_| CheckerError::ScriptError("checker_key_cache_failed".to_owned()))?
            .get(&challenge_id)
            .cloned()
        {
            return Ok(key);
        }
        let data = self
            .media
            .get(format!("challenges/{challenge_id}"), ".key".to_owned())
            .await?;
        let key = modules::fs::decode_key(data)
            .map_err(|error| CheckerError::ScriptError(error.to_owned()))?;
        self.key_cache
            .write()
            .map_err(|_| CheckerError::ScriptError("checker_key_cache_failed".to_owned()))?
            .insert(challenge_id, key.clone());
        Ok(key)
    }

    pub async fn lint(&self, challenge: &cds_db::Challenge) -> Result<(), CheckerError> {
        let script = challenge
            .checker
            .as_deref()
            .ok_or_else(|| CheckerError::MissingScript(String::new()))?;
        let configure = self.configure_lua(challenge.id, None);
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
        let configure =
            self.configure_lua(challenge.id, Some(self.default_key(challenge.id).await?));
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
        let configure =
            self.configure_lua(challenge.id, Some(self.default_key(challenge.id).await?));
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
    const REGEX: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/regex.lua"
    );
    const SUID: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/suid.lua"
    );
    const SUID_CUSTOM_KEY: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/suid-custom-key.lua"
    );
    const LEET: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/leet.lua"
    );
    const LEET_CUSTOM_KEY: &str = include_str!(
        "../../../web/src/pages/admin/challenges/challenge_id/checker/_blocks/examples/leet-custom-key.lua"
    );

    fn configure() -> Arc<ConfigureLua> {
        Arc::new(|lua: &Lua| {
            modules::audit::install(lua)?;
            modules::suid::install(lua, Some("11".repeat(64)))?;
            modules::leet::install(lua, Some("11".repeat(64)))?;
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
        for script in [SIMPLE, REGEX, SUID, SUID_CUSTOM_KEY, LEET, LEET_CUSTOM_KEY] {
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

        cds_engine::preload("test/regex", REGEX, None)
            .await
            .unwrap();
        let correct: StatusOutput = cds_engine::execute(
            "test/regex",
            "check",
            (1_i64, "flag{this_is_my_flag_2026}"),
            configure.as_ref(),
        )
        .await
        .unwrap();
        assert_eq!(Status::try_from(correct).unwrap(), Status::Correct);
        let incorrect: StatusOutput = cds_engine::execute(
            "test/regex",
            "check",
            (1_i64, "flag{this_is_my_flag}"),
            configure.as_ref(),
        )
        .await
        .unwrap();
        assert_eq!(Status::try_from(incorrect).unwrap(), Status::Incorrect);

        for (key, script) in [
            ("test/suid", SUID),
            ("test/suid-custom-key", SUID_CUSTOM_KEY),
            ("test/leet", LEET),
            ("test/leet-custom-key", LEET_CUSTOM_KEY),
        ] {
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

    #[tokio::test]
    async fn generic_checker_libraries_are_available_at_top_level() {
        let configure = configure();
        let script = r#"
            function value()
                return {
                    digest = crypto.sha256("answer"),
                    encoded = http.url_encode("hello world"),
                    json = json.encode({ answer = 42 }),
                    matched = tostring(regex.is_match("^ans", "answer")),
                    request_type = type(http.request)
                }
            end
        "#;
        cds_engine::preload("test/global-checker-libraries", script, None)
            .await
            .unwrap();
        let result: HashMap<String, String> = cds_engine::execute(
            "test/global-checker-libraries",
            "value",
            (),
            configure.as_ref(),
        )
        .await
        .unwrap();
        assert_eq!(
            result["digest"],
            cds_engine::modules::crypto::sha256("answer")
        );
        assert_eq!(result["encoded"], "hello+world");
        assert_eq!(result["json"], r#"{"answer":42}"#);
        assert_eq!(result["matched"], "true");
        assert_eq!(result["request_type"], "function");
    }

    #[tokio::test]
    async fn leet_and_suid_accept_custom_key_options() {
        let configure = configure();
        let script = r#"
            local custom_key = "custom-key-owned-by-script"
            function value()
                local leet = checker.leet.encode("answer", 42, { key = custom_key })
                local suid = checker.suid.encode(42, {
                    key = custom_key,
                    hyphenated = true
                })
                return {
                    leet = tostring(checker.leet.decode("answer", leet, { key = custom_key })),
                    suid = tostring(checker.suid.decode(suid, { key = custom_key })),
                    hyphenated = tostring(string.find(suid, "-", 1, true) ~= nil)
                }
            end
        "#;
        cds_engine::preload("test/custom-checker-key", script, None)
            .await
            .unwrap();
        let result: HashMap<String, String> =
            cds_engine::execute("test/custom-checker-key", "value", (), configure.as_ref())
                .await
                .unwrap();
        assert_eq!(result["leet"], "42");
        assert_eq!(result["suid"], "42");
        assert_eq!(result["hyphenated"], "true");
    }
}

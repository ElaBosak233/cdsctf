//! ChallengeDetail checker powered by the embedded Lua engine.
//!
//! Scripts expose top-level `check` and `generate` functions. Checker-specific
//! APIs are available under the `checker` global namespace.

pub mod modules;
mod subject;
pub mod traits;
pub mod util;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use cds_engine::{ConfigureLua, mlua::Lua};
use cds_media::Media;
use serde::Deserialize;
use tracing::debug;

use crate::traits::CheckerError;
pub use crate::{modules::audit::Status, subject::Subject};

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

    pub async fn lint(&self, challenge: &cds_db::ChallengeDetail) -> Result<(), CheckerError> {
        let script = challenge
            .checker
            .as_deref()
            .ok_or_else(|| CheckerError::MissingScript(String::new()))?;
        let configure = self.configure_lua(challenge.id, None);
        cds_engine::lint(script, &["check", "generate"], configure.as_ref()).await?;
        Ok(())
    }

    async fn preload(&self, challenge: &cds_db::ChallengeDetail) -> Result<(), CheckerError> {
        cds_engine::preload(
            format!("challenge/{}", challenge.id),
            challenge
                .checker
                .as_deref()
                .ok_or_else(|| CheckerError::MissingScript(String::new()))?,
            Some(challenge.updated_at),
        )
        .await?;
        Ok(())
    }

    pub async fn check(
        &self,
        challenge: &cds_db::ChallengeDetail,
        subject: Subject,
        content: &str,
    ) -> Result<Status, CheckerError> {
        let subject_id = subject.subject_id()?;
        self.preload(challenge).await?;
        debug!(
            challenge_id = challenge.id,
            subject_id, "Checking answer with Lua"
        );
        let configure =
            self.configure_lua(challenge.id, Some(self.default_key(challenge.id).await?));
        let result: StatusOutput = cds_engine::execute(
            format!("challenge/{}", challenge.id),
            "check",
            (subject_id, content),
            configure.as_ref(),
        )
        .await?;
        result.into_status(subject)
    }

    pub async fn generate(
        &self,
        challenge: &cds_db::ChallengeDetail,
        subject: Subject,
    ) -> Result<HashMap<String, String>, CheckerError> {
        let subject_id = subject.subject_id()?;
        self.preload(challenge).await?;
        debug!(
            challenge_id = challenge.id,
            subject_id, "Generating environment variables"
        );
        let configure =
            self.configure_lua(challenge.id, Some(self.default_key(challenge.id).await?));
        Ok(cds_engine::execute(
            format!("challenge/{}", challenge.id),
            "generate",
            (subject_id,),
            configure.as_ref(),
        )
        .await?)
    }
}

#[derive(Deserialize)]
struct StatusOutput {
    kind: String,
    #[serde(alias = "operator_id")]
    subject_id: Option<i64>,
}

impl StatusOutput {
    fn into_status(self, subject: Subject) -> Result<Status, CheckerError> {
        match self.kind.as_str() {
            "correct" => Ok(Status::Correct),
            "incorrect" => Ok(Status::Incorrect),
            "cheat" => {
                let subject_id = self.subject_id.ok_or_else(|| {
                    CheckerError::ScriptError("cheat status requires subject_id".to_owned())
                })?;
                // Only another team's flag can enter team cheat policy. A user
                // flag (including a same-numbered user) is simply incorrect.
                Ok(match (subject, Subject::from_subject_id(subject_id)) {
                    (Subject::Team(team_id), Some(Subject::Team(peer_team_id)))
                        if team_id != peer_team_id =>
                    {
                        Status::Cheat(peer_team_id)
                    }
                    _ => Status::Incorrect,
                })
            }
            _ => Err(CheckerError::ScriptError(format!(
                "unknown checker status: {}",
                self.kind
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use cds_engine::{ConfigureLua, mlua::Lua};

    use super::{Status, StatusOutput, Subject, modules};

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
        for (subject, source, expected) in [
            (Subject::Team(7), 42, Status::Cheat(42)),
            (Subject::Team(7), -42, Status::Incorrect),
            (Subject::Team(7), -7, Status::Incorrect),
            (Subject::Team(7), 7, Status::Incorrect),
            (Subject::Team(7), 0, Status::Incorrect),
            (Subject::Team(7), i64::MIN, Status::Incorrect),
            (Subject::User(7), 42, Status::Incorrect),
            (Subject::User(7), 7, Status::Incorrect),
            (Subject::User(7), -42, Status::Incorrect),
        ] {
            let status = StatusOutput {
                kind: "cheat".to_owned(),
                subject_id: Some(source),
            };
            assert_eq!(status.into_status(subject).unwrap(), expected);
        }
        for (kind, source) in [("cheat", None), ("unknown", Some(42))] {
            assert!(
                StatusOutput {
                    kind: kind.to_owned(),
                    subject_id: source,
                }
                .into_status(Subject::Team(7))
                .is_err()
            );
        }
    }

    #[tokio::test]
    async fn legacy_script_parameters_and_return_fields_remain_supported() {
        let configure = configure();
        let key = "test/legacy-checker-subject";
        let script = r#"
            function generate(operator_id)
                return { FLAG = checker.suid.encode(operator_id) }
            end

            function check(operator_id, content)
                local source = checker.suid.decode(content)
                if source == operator_id then
                    return checker.audit.correct()
                end
                return { kind = "cheat", operator_id = source }
            end
        "#;
        cds_engine::preload(key, script, None).await.unwrap();
        for owner in [Subject::User(7), Subject::Team(7)] {
            let generated: HashMap<String, String> = cds_engine::execute(
                key,
                "generate",
                (owner.subject_id().unwrap(),),
                configure.as_ref(),
            )
            .await
            .unwrap();
            for submitter in [Subject::User(7), Subject::Team(7), Subject::Team(8)] {
                let output: StatusOutput = cds_engine::execute(
                    key,
                    "check",
                    (submitter.subject_id().unwrap(), generated["FLAG"].as_str()),
                    configure.as_ref(),
                )
                .await
                .unwrap();
                let expected = match (owner, submitter) {
                    _ if owner == submitter => Status::Correct,
                    (Subject::Team(source), Subject::Team(_)) => Status::Cheat(source),
                    _ => Status::Incorrect,
                };
                if owner != submitter {
                    assert_eq!(output.subject_id, Some(owner.subject_id().unwrap()));
                }
                assert_eq!(output.into_status(submitter).unwrap(), expected);
            }
        }
    }

    #[tokio::test]
    async fn bundled_templates_lint_and_execute() {
        let configure = configure();
        for script in [SIMPLE, REGEX, SUID, SUID_CUSTOM_KEY, LEET, LEET_CUSTOM_KEY] {
            cds_engine::lint(script, &["check", "generate"], configure.as_ref())
                .await
                .unwrap();
        }

        for (key, script, valid, invalid) in [
            (
                "test/simple",
                SIMPLE,
                "flag{this_is_my_flag}",
                "flag{wrong}",
            ),
            (
                "test/regex",
                REGEX,
                "flag{this_is_my_flag_2026}",
                "flag{this_is_my_flag}",
            ),
        ] {
            cds_engine::preload(key, script, None).await.unwrap();
            for subject in [
                Subject::User(1),
                Subject::Team(1),
                Subject::User(i64::MAX),
                Subject::Team(i64::MAX),
            ] {
                let generated: HashMap<String, String> = cds_engine::execute(
                    key,
                    "generate",
                    (subject.subject_id().unwrap(),),
                    configure.as_ref(),
                )
                .await
                .unwrap();
                assert!(generated.is_empty());
                for (content, expected) in [
                    (valid, Status::Correct),
                    (invalid, Status::Incorrect),
                    ("not a flag", Status::Incorrect),
                ] {
                    let output: StatusOutput = cds_engine::execute(
                        key,
                        "check",
                        (subject.subject_id().unwrap(), content),
                        configure.as_ref(),
                    )
                    .await
                    .unwrap();
                    assert_eq!(
                        output.into_status(subject).unwrap(),
                        expected,
                        "{key}: {subject:?}"
                    );
                }
            }
        }

        for (key, script) in [
            ("test/suid", SUID),
            ("test/suid-custom-key", SUID_CUSTOM_KEY),
            ("test/leet", LEET),
            ("test/leet-custom-key", LEET_CUSTOM_KEY),
        ] {
            cds_engine::preload(key, script, None).await.unwrap();
            for id in [1, 7, i64::MAX] {
                for owner in [Subject::User(id), Subject::Team(id)] {
                    let mut generated: HashMap<String, String> = cds_engine::execute(
                        key,
                        "generate",
                        (owner.subject_id().unwrap(),),
                        configure.as_ref(),
                    )
                    .await
                    .unwrap();
                    let flag = generated.remove("FLAG").unwrap();
                    let other_id = if id == 1 { 2 } else { 1 };
                    for submitter in [
                        owner,
                        Subject::User(id),
                        Subject::Team(id),
                        Subject::User(other_id),
                        Subject::Team(other_id),
                    ] {
                        let output: StatusOutput = cds_engine::execute(
                            key,
                            "check",
                            (submitter.subject_id().unwrap(), flag.as_str()),
                            configure.as_ref(),
                        )
                        .await
                        .unwrap();
                        // The unchanged scripts must recover the complete
                        // signed subject before the
                        // Rust boundary applies team policy.
                        if submitter != owner {
                            assert_eq!(output.kind, "cheat", "{key}: {owner:?} -> {submitter:?}");
                            assert_eq!(output.subject_id, Some(owner.subject_id().unwrap()));
                        }
                        let expected = match (owner, submitter) {
                            _ if owner == submitter => Status::Correct,
                            (Subject::Team(source), Subject::Team(_)) => Status::Cheat(source),
                            _ => Status::Incorrect,
                        };
                        assert_eq!(
                            output.into_status(submitter).unwrap(),
                            expected,
                            "{key}: {owner:?} -> {submitter:?}"
                        );
                    }
                    for invalid in [
                        "not a flag".to_owned(),
                        "flag{broken}".to_owned(),
                        flag.replacen("flag{", "wrong{", 1),
                    ] {
                        let output: StatusOutput = cds_engine::execute(
                            key,
                            "check",
                            (owner.subject_id().unwrap(), invalid.as_str()),
                            configure.as_ref(),
                        )
                        .await
                        .unwrap();
                        assert_eq!(
                            output.into_status(owner).unwrap(),
                            Status::Incorrect,
                            "{key}: {owner:?}"
                        );
                    }
                }
            }
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

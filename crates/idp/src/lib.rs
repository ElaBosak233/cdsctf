//! Lua-backed identity provider adapters.
//!
//! Scripts expose top-level `login(params)` and `bind(params, user)` functions.
//! JSON and HTTP integrations are available as top-level `json` and `http`
//! libraries.

use std::{collections::HashMap, sync::Arc};

use cds_engine::{ConfigureLua, mlua::Lua, traits::EngineError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IdentityPayload {
    pub auth_key: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub data: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum IdpError {
    #[error("missing idp field: {0}")]
    MissingField(String),
    #[error("invalid idp field: {0}")]
    InvalidField(String),
    #[error("script error: {0}")]
    ScriptError(String),
    #[error("engine error: {0}")]
    EngineError(#[from] EngineError),
}

#[derive(Clone, Debug, Default)]
pub struct Idp;

impl Idp {
    fn configure_lua() -> Arc<ConfigureLua> {
        Arc::new(|_lua: &Lua| Ok(()))
    }

    fn key(idp_id: impl ToString) -> String {
        format!("idp/{}", idp_id.to_string())
    }

    pub async fn lint(script: impl AsRef<str>) -> Result<(), EngineError> {
        let configure = Self::configure_lua();
        cds_engine::lint(script, &["login", "bind"], configure.as_ref()).await
    }

    pub async fn preload(
        idp_id: impl ToString,
        script: impl AsRef<str>,
    ) -> Result<(), EngineError> {
        cds_engine::preload(Self::key(idp_id), script, None).await
    }

    pub async fn login(
        idp_id: impl ToString,
        params: HashMap<String, String>,
    ) -> Result<IdentityPayload, IdpError> {
        let configure = Self::configure_lua();
        let result: HashMap<String, String> = cds_engine::execute_json(
            Self::key(idp_id),
            "login",
            &[serde_json::to_value(params)
                .map_err(|error| EngineError::OtherError(error.into()))?],
            configure.as_ref(),
        )
        .await?;
        Self::decode_payload(result)
    }

    pub async fn bind(
        idp_id: impl ToString,
        params: HashMap<String, String>,
        user: HashMap<String, String>,
    ) -> Result<IdentityPayload, IdpError> {
        let configure = Self::configure_lua();
        let result: HashMap<String, String> = cds_engine::execute_json(
            Self::key(idp_id),
            "bind",
            &[
                serde_json::to_value(params)
                    .map_err(|error| EngineError::OtherError(error.into()))?,
                serde_json::to_value(user)
                    .map_err(|error| EngineError::OtherError(error.into()))?,
            ],
            configure.as_ref(),
        )
        .await?;
        Self::decode_payload(result)
    }

    fn decode_payload(data: HashMap<String, String>) -> Result<IdentityPayload, IdpError> {
        let auth_key = data
            .get("auth_key")
            .cloned()
            .ok_or_else(|| IdpError::MissingField("auth_key".to_owned()))?;
        if auth_key.is_empty() || auth_key.len() > 255 {
            return Err(IdpError::InvalidField("auth_key".to_owned()));
        }
        Ok(IdentityPayload { auth_key, data })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Idp;

    const DEFAULT: &str =
        include_str!("../../../web/src/pages/admin/idps/idp_id/_blocks/examples/default.lua");
    const GITHUB: &str =
        include_str!("../../../web/src/pages/admin/idps/idp_id/_blocks/examples/github.lua");
    const CAS: &str =
        include_str!("../../../web/src/pages/admin/idps/idp_id/_blocks/examples/cas.lua");

    #[test]
    fn decodes_identity_payload() {
        let payload = Idp::decode_payload(HashMap::from([
            ("auth_key".to_owned(), "42".to_owned()),
            ("username".to_owned(), "ela".to_owned()),
        ]))
        .unwrap();
        assert_eq!(payload.auth_key, "42");
        assert_eq!(payload.data["username"], "ela");
    }

    #[test]
    fn rejects_invalid_auth_keys() {
        assert!(
            Idp::decode_payload(HashMap::from([("auth_key".to_owned(), "".to_owned())])).is_err()
        );
        assert!(
            Idp::decode_payload(HashMap::from([("auth_key".to_owned(), "x".repeat(256),)]))
                .is_err()
        );
    }

    #[tokio::test]
    async fn scripts_can_use_global_libraries() {
        cds_engine::clear_cache();
        let script = "function login(params) return { auth_key = json.encode(params) } end function bind(params, user) return login(params) end";
        Idp::preload("global-libraries", script).await.unwrap();
        let payload = Idp::login(
            "global-libraries",
            HashMap::from([("answer".to_owned(), "42".to_owned())]),
        )
        .await
        .unwrap();
        assert_eq!(payload.auth_key, r#"{"answer":"42"}"#);
    }

    #[tokio::test]
    async fn bundled_templates_lint() {
        for script in [DEFAULT, GITHUB, CAS] {
            Idp::lint(script).await.unwrap();
        }
    }
}

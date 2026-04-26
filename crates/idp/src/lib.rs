//! Rune-backed identity provider adapters.
//!
//! The platform owns persistence and sessions, while provider-specific
//! authentication is delegated to Rune scripts. Scripts expose `login(params)`
//! and `bind(params, user)` and must return an object containing `auth_key`.

use std::collections::HashMap;

use cds_engine::{
    rune::{self, Any, ContextError, Module, Value, runtime::Object},
    rune_modules,
    traits::EngineError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Any)]
#[rune(item = ::cds::idp)]
pub struct RuneMap(pub HashMap<String, String>);

impl RuneMap {
    #[rune::function(path = Self::get)]
    pub fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    #[rune::function(path = Self::contains_key)]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
}

#[rune::module(::cds::idp)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(self::module_meta)?;
    module.ty::<RuneMap>()?;
    module.function_meta(RuneMap::get)?;
    module.function_meta(RuneMap::contains_key)?;
    Ok(module)
}

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
    #[error("script error: {0}")]
    ScriptError(String),
    #[error("engine error: {0}")]
    EngineError(#[from] EngineError),
}

#[derive(Clone, Debug, Default)]
pub struct Idp;

impl Idp {
    async fn context() -> Result<cds_engine::rune::Context, EngineError> {
        cds_engine::gen_rune_context(vec![
            rune_modules::http::module(true).map_err(EngineError::from)?,
            rune_modules::json::module(true).map_err(EngineError::from)?,
            rune_modules::toml::module(true).map_err(EngineError::from)?,
            rune_modules::process::module(true).map_err(EngineError::from)?,
            module(true).map_err(EngineError::from)?,
        ])
        .await
    }

    fn key(idp_id: impl ToString) -> String {
        format!("idp/{}", idp_id.to_string())
    }

    pub async fn lint(script: impl AsRef<str>) -> Result<(), EngineError> {
        cds_engine::lint(Self::context().await?, script, &["login", "bind"]).await
    }

    pub async fn preload(
        idp_id: impl ToString,
        script: impl AsRef<str>,
    ) -> Result<(), EngineError> {
        cds_engine::preload(Self::context().await?, Self::key(idp_id), script, None).await
    }

    pub async fn login(
        idp_id: impl ToString,
        params: HashMap<String, String>,
    ) -> Result<IdentityPayload, IdpError> {
        let result = cds_engine::execute(Self::key(idp_id), "login", (RuneMap(params),)).await?;
        Self::decode_payload(result)
    }

    pub async fn bind(
        idp_id: impl ToString,
        params: HashMap<String, String>,
        user: HashMap<String, String>,
    ) -> Result<IdentityPayload, IdpError> {
        let result =
            cds_engine::execute(Self::key(idp_id), "bind", (RuneMap(params), RuneMap(user)))
                .await?;
        Self::decode_payload(result)
    }

    fn decode_payload(value: Value) -> Result<IdentityPayload, IdpError> {
        let output: Result<Object, Value> = rune::from_value(value).map_err(EngineError::from)?;
        let object = output.map_err(|err| IdpError::ScriptError(format!("{err:?}")))?;

        println!("Decoded payload: {object:?}");

        let mut data: HashMap<String, String> = HashMap::new();
        for (key, value) in object.iter() {
            data.insert(
                key.to_string(),
                rune::from_value(value.clone()).map_err(EngineError::from)?,
            );
        }

        let auth_key = data.get("auth_key").ok_or_else(|| IdpError::MissingField("auth_key".to_owned()))?.to_owned();

        Ok(IdentityPayload { auth_key, data })
    }
}

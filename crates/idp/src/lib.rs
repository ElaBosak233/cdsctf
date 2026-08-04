//! Lua-backed identity provider adapters.
//!
//! Scripts expose top-level `login(params)` and `bind(params, user)` functions.
//! JSON and HTTP integrations are available under the `cds` namespace.

use std::{collections::HashMap, sync::Arc, time::Duration};

use cds_engine::{
    ConfigureLua,
    mlua::{ExternalResult, Lua, LuaSerdeExt, Table, Value},
    traits::EngineError,
};
use reqwest::{Method, redirect::Policy};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const MAX_HTTP_BODY_SIZE: usize = 2 * 1024 * 1024;

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
    fn configure_lua() -> Arc<ConfigureLua> {
        Arc::new(|lua: &Lua| {
            install_json(lua)?;
            install_http(lua)?;
            Ok(())
        })
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
        Ok(IdentityPayload { auth_key, data })
    }
}

fn install_json(lua: &Lua) -> Result<(), EngineError> {
    let module = cds_engine::module(lua, "json")?;
    module.set(
        "encode",
        lua.create_function(|lua, value: Value| {
            let value: serde_json::Value = lua.from_value(value)?;
            serde_json::to_string(&value).into_lua_err()
        })?,
    )?;
    module.set(
        "decode",
        lua.create_function(|lua, value: String| {
            let value: serde_json::Value = serde_json::from_str(&value).into_lua_err()?;
            lua.to_value(&value)
        })?,
    )?;
    Ok(())
}

fn install_http(lua: &Lua) -> Result<(), EngineError> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .redirect(Policy::limited(5))
        .build()
        .map_err(anyhow::Error::from)?;
    let module = cds_engine::module(lua, "http")?;
    module.set(
        "url_encode",
        lua.create_function(|_, value: String| {
            Ok(url::form_urlencoded::byte_serialize(value.as_bytes()).collect::<String>())
        })?,
    )?;
    module.set(
        "request",
        lua.create_async_function(
            move |lua,
                  (method, url, headers, body): (
                String,
                String,
                Option<Table>,
                Option<String>,
            )| {
                let client = client.clone();
                async move {
                    if !(url.starts_with("https://") || url.starts_with("http://")) {
                        return Err(mlua::Error::RuntimeError(
                            "http only supports http and https URLs".to_owned(),
                        ));
                    }
                    let method = Method::from_bytes(method.as_bytes()).into_lua_err()?;
                    let mut request = client.request(method, url);
                    if let Some(headers) = headers {
                        for pair in headers.pairs::<String, String>() {
                            let (name, value) = pair?;
                            request = request.header(name, value);
                        }
                    }
                    if let Some(body) = body {
                        request = request.body(body);
                    }
                    let mut response = request.send().await.into_lua_err()?;
                    if response
                        .content_length()
                        .is_some_and(|length| length > MAX_HTTP_BODY_SIZE as u64)
                    {
                        return Err(mlua::Error::RuntimeError(
                            "http response body is too large".to_owned(),
                        ));
                    }

                    let status = response.status().as_u16();
                    let response_headers = response.headers().clone();
                    let mut body = Vec::with_capacity(
                        response
                            .content_length()
                            .unwrap_or_default()
                            .min(MAX_HTTP_BODY_SIZE as u64) as usize,
                    );
                    while let Some(chunk) = response.chunk().await.into_lua_err()? {
                        if chunk.len() > MAX_HTTP_BODY_SIZE.saturating_sub(body.len()) {
                            return Err(mlua::Error::RuntimeError(
                                "http response body is too large".to_owned(),
                            ));
                        }
                        body.extend_from_slice(&chunk);
                    }
                    let body = String::from_utf8(body).into_lua_err()?;
                    let result = lua.create_table()?;
                    result.set("status", status)?;
                    result.set("body", body)?;
                    let headers = lua.create_table()?;
                    for (name, value) in &response_headers {
                        if let Ok(value) = value.to_str() {
                            headers.set(name.as_str(), value)?;
                        }
                    }
                    result.set("headers", headers)?;
                    Ok(result)
                }
            },
        )?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cds_engine::mlua::{Function, Lua, Value};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    use super::{Idp, install_http, install_json};

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
    fn json_module_round_trips() {
        let lua = Lua::new();
        lua.globals()
            .set("cds", lua.create_table().unwrap())
            .unwrap();
        install_json(&lua).unwrap();
        let value: String = lua
            .load(r#"return cds.json.encode({ answer = 42 })"#)
            .eval()
            .unwrap();
        let decoded: HashMap<String, i64> = serde_json::from_str(&value).unwrap();
        assert_eq!(decoded["answer"], 42);
    }

    #[tokio::test]
    async fn http_module_rejects_oversized_chunked_body() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request).await;
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n")
                .await
                .unwrap();
            let chunk = vec![b'x'; 1024 * 1024];
            for _ in 0..2 {
                stream.write_all(b"100000\r\n").await.unwrap();
                stream.write_all(&chunk).await.unwrap();
                stream.write_all(b"\r\n").await.unwrap();
            }
            let _ = stream.write_all(b"1\r\nx\r\n0\r\n\r\n").await;
        });

        let lua = cds_engine::create_lua().unwrap();
        install_http(&lua).unwrap();
        let request: Function = lua
            .load("return function(url) return cds.http.request('GET', url, nil, nil) end")
            .eval()
            .unwrap();
        let error = request
            .call_async::<Value>(format!("http://{address}"))
            .await
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("http response body is too large")
        );
        server.abort();
    }

    #[tokio::test]
    async fn bundled_templates_lint() {
        let configure = |lua: &Lua| {
            install_json(lua)?;
            install_http(lua)?;
            Ok(())
        };
        for script in [DEFAULT, GITHUB, CAS] {
            cds_engine::lint(script, &["login", "bind"], &configure)
                .await
                .unwrap();
        }
    }
}

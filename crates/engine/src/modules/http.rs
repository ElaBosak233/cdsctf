//! Global asynchronous Lua module `http`.

use std::time::Duration;

use mlua::{ExternalResult, Lua, Table};
use reqwest::{Method, redirect::Policy};

use crate::{global_module, traits::EngineError};

const MAX_HTTP_BODY_SIZE: usize = 2 * 1024 * 1024;

pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .redirect(Policy::limited(5))
        .build()
        .map_err(anyhow::Error::from)?;
    let module = global_module(lua, "http")?;
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
    use mlua::{Function, Value};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    use crate::create_lua;

    #[test]
    fn url_encodes_values() {
        let lua = create_lua().unwrap();
        let value: String = lua
            .load("return http.url_encode('hello world&answer=42')")
            .eval()
            .unwrap();
        assert_eq!(value, "hello+world%26answer%3D42");
    }

    #[tokio::test]
    async fn rejects_oversized_chunked_bodies() {
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

        let lua = create_lua().unwrap();
        let request: Function = lua
            .load("return function(url) return http.request('GET', url, nil, nil) end")
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
}

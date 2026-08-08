//! Global asynchronous Lua module `time`.

use std::time::Duration;

use mlua::Lua;

use crate::{global_module, traits::EngineError};

/// Installs the time helpers available to every Lua script.
pub(crate) fn install(lua: &Lua) -> Result<(), EngineError> {
    let module = global_module(lua, "time")?;
    module.set(
        "sleep",
        lua.create_async_function(|_, seconds: f64| async move {
            let duration = Duration::try_from_secs_f64(seconds).map_err(|_| {
                mlua::Error::RuntimeError(
                    "time.sleep expects a finite, non-negative number of seconds".to_owned(),
                )
            })?;
            tokio::time::sleep(duration).await;
            Ok(())
        })?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use mlua::{Function, Value};

    use crate::create_lua;

    #[tokio::test]
    async fn sleeps_without_blocking_the_lua_call() {
        let lua = create_lua().unwrap();
        let sleep: Function = lua
            .load("return function() return time.sleep(0.03) end")
            .eval()
            .unwrap();
        let started = Instant::now();
        let result = sleep.call_async::<Value>(()).await.unwrap();
        assert!(result.is_nil());
        assert!(started.elapsed().as_millis() >= 20);
    }

    #[tokio::test]
    async fn rejects_invalid_sleep_durations() {
        let lua = create_lua().unwrap();
        for expression in ["-1", "0 / 0", "math.huge"] {
            let sleep: Function = lua
                .load(format!(
                    "return function() return time.sleep({expression}) end"
                ))
                .eval()
                .unwrap();
            let error = sleep.call_async::<Value>(()).await.unwrap_err();
            assert!(error.to_string().contains("time.sleep expects"));
        }
    }
}

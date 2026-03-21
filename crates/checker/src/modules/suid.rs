//! Rune built-in module `suid` for challenge checker scripts.

use cds_engine::{
    rune,
    rune::{ContextError, Module},
};

/// Constructs the Rune native module exposed to checker scripts.
#[rune::module(::suid)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(encode)?;
    module.function_meta(decode)?;

    Ok(module)
}

/// Encodes a flag payload using the module's algorithm.
#[rune::function]
pub fn encode(data: i64, key: &str, hyphenated: bool) -> String {
    crate::util::suid::encode(data, key, hyphenated)
}

/// Decodes a payload produced by [`encode`].
#[rune::function]
pub fn decode(payload: &str, key: &str) -> Result<i64, anyhow::Error> {
    crate::util::suid::decode(payload, key)
}

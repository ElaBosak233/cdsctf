//! Rune built-in module `leet` for challenge checker scripts.

use std::io;

use cds_engine::{
    rune,
    rune::{ContextError, Module},
};

/// Constructs the Rune native module exposed to checker scripts.
#[rune::module(::leet)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(encode)?;
    module.function_meta(decode)?;

    Ok(module)
}

/// Encodes a flag payload using the module's algorithm.
#[rune::function]
pub fn encode(template: &str, data: i64, key: &str) -> String {
    crate::util::leet::encode(template, data, key)
}

/// Decodes a payload produced by [`encode`].
#[rune::function]
pub fn decode(template: &str, payload: &str, key: &str) -> Result<i64, io::Error> {
    crate::util::leet::decode(template, payload, key)
}

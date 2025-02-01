use std::io;

use rune::{ContextError, Module};

#[rune::module(::leet)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(encode)?;
    module.function_meta(decode)?;

    Ok(module)
}

#[rune::function]
pub fn encode(template: &str, data: i64, key: &str) -> String {
    crate::util::leet::encode(template, data, key)
}

#[rune::function]
pub fn decode(template: &str, payload: &str, key: &str) -> Result<i64, io::Error> {
    crate::util::leet::decode(template, payload, key)
}

use rune::{ContextError, Module};

#[rune::module(::suid)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(encode)?;
    module.function_meta(decode)?;

    Ok(module)
}

#[rune::function]
pub fn encode(seed: &str, data: i64, key: &str, hyphenated: bool) -> String {
    crate::util::suid::encode(seed, data, key, hyphenated)
}

#[rune::function]
pub fn decode(seed: &str, payload: &str, key: &str) -> Result<i64, anyhow::Error> {
    crate::util::suid::decode(seed, payload, key)
}

use ring::digest::{SHA256, SHA512};
use rune::{ContextError, Module};

#[rune::module(::crypto)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(sha256)?;
    module.function_meta(sha512)?;

    Ok(module)
}

#[rune::function]
pub fn sha256(message: &str) -> String {
    let mut context = ring::digest::Context::new(&SHA256);
    context.update(message.as_bytes());
    hex::encode(context.finish().as_ref())
}

#[rune::function]
pub fn sha512(message: &str) -> String {
    let mut context = ring::digest::Context::new(&SHA512);
    context.update(message.as_bytes());
    hex::encode(context.finish().as_ref())
}

//! Rune built-in module `crypto` for challenge checker scripts.

use cds_engine::{
    rune,
    rune::{ContextError, Module},
};
use ring::digest::{SHA256, SHA512};

/// Constructs the Rune native module exposed to checker scripts.
#[rune::module(::crypto)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(sha256)?;
    module.function_meta(sha512)?;

    Ok(module)
}

/// Computes a SHA-256 digest from Rune strings or bytes.
#[rune::function]
pub fn sha256(message: &str) -> String {
    let mut context = ring::digest::Context::new(&SHA256);
    context.update(message.as_bytes());
    hex::encode(context.finish().as_ref())
}

/// Computes a SHA-512 digest from Rune strings or bytes.
#[rune::function]
pub fn sha512(message: &str) -> String {
    let mut context = ring::digest::Context::new(&SHA512);
    context.update(message.as_bytes());
    hex::encode(context.finish().as_ref())
}

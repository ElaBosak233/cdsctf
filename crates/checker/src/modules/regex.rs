//! Rune built-in module `regex` for challenge checker scripts.

use cds_engine::{
    rune,
    rune::{ContextError, Module},
};

/// Constructs the Rune native module exposed to checker scripts.
#[rune::module(::regex)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(is_match)?;

    Ok(module)
}

/// Returns whether is match.

#[rune::function]
pub fn is_match(pattern: &str, payload: &str) -> bool {
    let re = regex::Regex::new(pattern);
    match re {
        Err(_) => false,
        Ok(re) => re.is_match(payload),
    }
}

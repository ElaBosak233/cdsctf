use cds_engine::{
    rune,
    rune::{ContextError, Module},
};

#[rune::module(::regex)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.function_meta(is_match)?;

    Ok(module)
}

#[rune::function]
pub fn is_match(pattern: &str, payload: &str) -> bool {
    let re = regex::Regex::new(pattern);
    match re {
        Err(_) => false,
        Ok(re) => re.is_match(payload),
    }
}

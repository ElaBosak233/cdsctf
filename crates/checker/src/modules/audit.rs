use std::{io, str::FromStr};

use cds_engine::{
    rune,
    rune::{Any, ContextError, Module},
};

#[rune::module(::audit)]
pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut module = Module::from_meta(module_meta)?;
    module.ty::<Status>()?;

    module.ty::<Flag>()?;
    module.function_meta(Flag::new)?;
    module.function_meta(Flag::parse)?;
    module.function_meta(Flag::with_prefix)?;
    module.function_meta(Flag::prefix)?;
    module.function_meta(Flag::with_content)?;
    module.function_meta(Flag::content)?;
    module.function_meta(Flag::format)?;

    Ok(module)
}

#[derive(Any, Debug, Clone)]
#[rune(item = ::audit)]
pub enum Status {
    #[rune(constructor)]
    Correct,
    #[rune(constructor)]
    Incorrect,
    #[rune(constructor)]
    Cheat(#[rune(get, set)] i64),
}

#[derive(Any, Debug, Clone)]
#[rune(item = ::audit)]
pub struct Flag {
    prefix: String,
    content: String,
}

impl Flag {
    #[rune::function(path = Self::new)]
    pub fn new() -> Self {
        Self {
            prefix: "".to_owned(),
            content: "".to_owned(),
        }
    }

    #[rune::function(path = Self::parse)]
    pub fn parse(f: &str) -> Result<Self, io::Error> {
        let f = f.trim();
        let prefix_end = f
            .find('{')
            .ok_or(io::Error::other("flag format is incorrect"))?;
        let prefix: String = f.chars().take(prefix_end).collect();
        let content = f.to_owned().replacen(&prefix, "", 1);
        if !(content.starts_with("{") && content.ends_with("}")) {
            return Err(io::Error::other("flag format is incorrect"))?;
        }
        let content = String::from_str(&content[1..(content.len() - 1)])
            .map_err(|_| io::Error::other("failed to extract flag content"))?;
        Ok(Self { prefix, content })
    }

    #[rune::function]
    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }

    #[rune::function]
    pub fn content(&self) -> String {
        self.content.clone()
    }

    #[rune::function]
    pub fn with_prefix(self, p: &str) -> Self {
        Self {
            prefix: p.to_owned(),
            ..self
        }
    }

    #[rune::function]
    pub fn with_content(self, c: &str) -> Self {
        Self {
            content: c.to_owned(),
            ..self
        }
    }

    #[rune::function]
    pub fn format(self) -> String {
        format!("{}{{{}}}", self.prefix, self.content)
    }
}

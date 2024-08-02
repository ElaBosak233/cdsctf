use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Default)]
pub struct Nat {
    pub src: String,
    pub dst: Option<String>,
    pub proxy: bool,
    pub entry: Option<String>,
}

impl Nat {
    pub fn simplify(&mut self) {
        if self.proxy {
            self.dst = None;
            self.entry = None;
        }
    }
}

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Default)]
pub struct Nat {
    pub src: String,
    pub dst: String,
    pub protocol: String,
    pub proxy: Option<String>,
    pub entry: Option<String>,
}

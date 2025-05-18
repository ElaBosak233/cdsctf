use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromJsonQueryResult, Eq, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub footer: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "CdsCTF".to_string(),
            description: "Stay determined".to_string(),
            keywords: vec!["CTF"].into_iter().map(|s| s.to_string()).collect(),
            footer: "Stay determined".to_string(),
        }
    }
}

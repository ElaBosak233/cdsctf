use sea_orm::{DeriveActiveEnum, EnumIter, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Port {
    pub value: i64,
    pub protocol: Protocol,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    EnumIter,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum Protocol {
    #[default]
    TCP = 0,
    UDP = 1,
}

impl Protocol {
    pub fn as_str(&self) -> &str {
        match self {
            Protocol::TCP => "tcp",
            Protocol::UDP => "udp",
        }
    }
}

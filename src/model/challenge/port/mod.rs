use sea_orm::{DeriveActiveEnum, EnumIter, FromJsonQueryResult};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Port {
    pub value: i64,
    pub protocol: Protocol,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
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

impl Serialize for Protocol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Protocol::TCP => 0,
            Protocol::UDP => 1,
        };
        return serializer.serialize_u8(value);
    }
}

impl<'de> Deserialize<'de> for Protocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Protocol::TCP),
            1 => Ok(Protocol::UDP),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown protocol type: {}",
                value
            ))),
        }
    }
}

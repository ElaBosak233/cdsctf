use sea_orm::{DeriveActiveEnum, EnumIter, FromJsonQueryResult};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Flag {
    #[serde(rename = "type")]
    pub type_: Type,
    pub banned: bool,
    pub env: Option<String>,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum Type {
    #[default]
    Static = 0,
    Pattern = 1,
    Dynamic = 2,
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Type::Static => 0,
            Type::Pattern => 1,
            Type::Dynamic => 2,
        };
        return serializer.serialize_u8(value);
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Type::Static),
            1 => Ok(Type::Pattern),
            2 => Ok(Type::Dynamic),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown flag type: {}",
                value
            ))),
        }
    }
}

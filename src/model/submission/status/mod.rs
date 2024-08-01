use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum Status {
    #[default]
    Pending = 0,
    Correct = 1,
    Incorrect = 2,
    Cheat = 3,
    Invalid = 4,
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Status::Pending => 0,
            Status::Correct => 1,
            Status::Incorrect => 2,
            Status::Cheat => 3,
            Status::Invalid => 4,
        };
        return serializer.serialize_u8(value);
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Status::Pending),
            1 => Ok(Status::Correct),
            2 => Ok(Status::Incorrect),
            3 => Ok(Status::Cheat),
            4 => Ok(Status::Invalid),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown status type: {}",
                value
            ))),
        }
    }
}

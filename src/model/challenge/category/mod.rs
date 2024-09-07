use sea_orm::{DeriveActiveEnum, EnumIter};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum Category {
    #[default]
    Uncategorized = 0,
    Misc = 1,
    Web = 2,
    Reverse = 3,
    Crypto = 4,
    Forensics = 5,
    Mobile = 6,
    Pwn = 7,
    Steganography = 8,
    Osint = 9,
    Hardware = 10,
    Cloud = 11,
    Societal = 12,
    Ai = 13,
    Blockchain = 14,
    Art = 15,
    Dev = 16,
}

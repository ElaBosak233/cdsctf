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
pub enum Group {
    #[default]
    Guest = 0,
    Banned = 1,
    User = 2,
    Admin = 3,
}

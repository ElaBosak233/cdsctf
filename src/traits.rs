use crate::model::user;

#[derive(Clone, Debug)]
pub struct Ext {
    pub operator: Option<user::Model>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

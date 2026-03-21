//! Object storage / media — `mod` (S3 and related helpers).

use crate::{
    Media,
    config::{email::Email, logo::Logo},
};

/// Defines the `email` submodule (see sibling `*.rs` files).
pub mod email;

/// Defines the `logo` submodule (see sibling `*.rs` files).
pub mod logo;

#[derive(Clone)]
pub struct Config<'a> {
    pub media: &'a Media,
}

impl<'a> Config<'a> {
    /// Constructs a new value.
    pub(crate) fn new(media: &'a Media) -> Self {
        Self { media }
    }

    /// Returns email-related media configuration views.
    pub fn email(&self) -> Email<'a> {
        Email::new(self.media)
    }

    /// Returns logo-related media configuration views.
    pub fn logo(&self) -> Logo<'a> {
        Logo::new(self.media)
    }
}

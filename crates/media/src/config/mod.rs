use crate::{
    Media,
    config::{email::Email, logo::Logo},
};

pub mod email;
pub mod logo;

#[derive(Clone)]
pub struct Config<'a> {
    pub media: &'a Media,
}

impl<'a> Config<'a> {
    pub(crate) fn new(media: &'a Media) -> Self {
        Self { media }
    }

    pub fn email(&self) -> Email<'a> {
        Email::new(self.media)
    }

    pub fn logo(&self) -> Logo<'a> {
        Logo::new(self.media)
    }
}
